package commands

import (
	"bufio"
	"context"
	"fmt"
	"github.com/jackc/pgx/v4/pgxpool"
	"log"
	"manuscript-core/client"
	"manuscript-core/pkg"
	"os"
	"os/signal"
	"regexp"
	"strings"
	"syscall"
)

var Provider = map[int]string{
	1: "Openai",
	2: "Gemini",
	3: "Gaia",
}

type LLMClient interface {
	GPTName() string
	SendRequest(prompt string) (string, error)
}

type TextToSQLRequest struct {
	Question string `json:"question"`
}

type TextToSQLResponse struct {
	Sql string `json:"sql"`
}

func generateModelPrompt() string {
	var sb strings.Builder
	sb.WriteString("Manuscript currently offers the following types of provider integration:\n")
	for i, name := range Provider {
		sb.WriteString(fmt.Sprintf("%d. %s\n", i, name))
	}
	sb.WriteString("Select model to use (default ChatGPT): ")
	return sb.String()
}

func Chat(manuscript string) {
	jobName := fmt.Sprintf("%s-postgres-1", manuscript)
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		fmt.Printf("Error: Failed to get postgres ps: %v", err)
		return
	}
	if len(dockers) == 0 {
		fmt.Println("No manuscript postgres found")
		return
	}
	for _, d := range dockers {
		if d.Name == jobName {
			manuscripts, err := pkg.LoadConfig(manuscriptConfig)
			if err != nil {
				log.Fatalf("Error: Failed to load manuscript config: %v", err)
			}
			for _, m := range manuscripts.Manuscripts {
				if m.Name == manuscript {
					prompt := generateModelPrompt()
					model := promptInput(prompt, "1")
					chat, err := newChatClient(model)
					if err != nil {
						log.Printf("Failed to create chat client: %v", err)
						return
					}

					ChatWithLLM(m, chat)
					break
				}
			}
			break
		}
	}
}

func newChatClient(model string) (LLMClient, error) {
	switch model {
	case "1":
		apiKey := os.Getenv("OPENAI_API_KEY")
		if apiKey == "" {
			return nil, fmt.Errorf("OPENAI_API_KEY environment variable not set, please set it to your OpenAI API key. You can obtain an API key from https://platform.openai.com, if you want change the model, please set `OPENAI_MODEL` environment variable to the model name")
		}
		return &client.ChatGPTClient{
			Name:    Provider[1],
			BaseURL: os.Getenv("OPENAI_API_BASE"),
			APIKey:  apiKey,
			Model:   os.Getenv("OPENAI_MODEL"),
		}, nil
	case "2":
		apiKey := os.Getenv("GEMINI_API_KEY")
		if apiKey == "" {
			return nil, fmt.Errorf("GEMINI_API_KEY environment variable not set, please set it to your Gemini API key. You can obtain an API key from https://ai.google.dev/gemini-api/docs/models/gemini, if you want change the model, please set `GEMINI_MODEL` environment variable to the model name")
		}
		return &client.GeminiClient{
			Name:   Provider[2],
			APIKey: apiKey,
			Model:  os.Getenv("GEMINI_MODEL"),
		}, nil
	case "3":
		baseUrl := os.Getenv("OPENAI_API_BASE")
		if baseUrl == "" {
			fmt.Printf("OPENAI_API_BASE environment variable not set, using default Gaia API base: %s\n", client.GaiaApiBase)
			baseUrl = client.GaiaApiBase
		}
		return &client.ChatGPTClient{
			Name:    Provider[3],
			BaseURL: baseUrl,
			APIKey:  os.Getenv("OPENAI_API_KEY"),
			Model:   "",
		}, nil
	default:
		return nil, fmt.Errorf("error: unknown model %s", model)
	}
}

func ChatWithLLM(job pkg.Manuscript, client LLMClient) {
	signalChannel := make(chan os.Signal, 1)
	signal.Notify(signalChannel, syscall.SIGINT, syscall.SIGTERM)

	pool, err := connectToDB(job)
	if err != nil {
		log.Fatal(err)
	}
	defer pool.Close()

	tableSchema := getTableInfo(pool)
	if tableSchema == "" {
		log.Fatalf("Error: No table found in the database")
	}

	reader := bufio.NewReader(os.Stdin)
	inputChannel := make(chan string)

	go func() {
		for {
			fmt.Print("🏄🏼‍You: ")
			userInput, _ := reader.ReadString('\n')
			userInput = strings.TrimSpace(userInput)
			inputChannel <- userInput
		}
	}()

	for {
		select {
		case userInput := <-inputChannel:
			if userInput == "exit" || userInput == "quit" {
				fmt.Println("Exiting chat.")
				return
			}

			prompt := fmt.Sprintf("The PostgreSQL database table with the following information:%s\n"+
				"The user's question is: '%s'\n"+
				"Please help user convert the question into SQL according to the above schema. Directly output plain text SQL, without any additional explanation or content."+
				"Note: If it's a SELECT * FROM query and there is no LIMIT keyword, proactively add a LIMIT 10; in all other cases, do not add the limit proactively.", tableSchema, userInput)

			fmt.Printf("Processing your question...\n")

			response, err := client.SendRequest(prompt)
			if err != nil {
				fmt.Printf("Error getting response: %v\n", err)
				continue
			}

			sqlQuery, err := extractSQL(response)
			if err != nil {
				fmt.Printf("Error extracting SQL: %v\n", err)
				continue
			}
			fmt.Printf("🔎🔎 \033[33m%s\033[0m: \u001B[32m%s\u001B[0m\nExecuting SQL......\n", client.GPTName(), sqlQuery)

			executeSQL(pool, sqlQuery)

		case <-signalChannel:
			fmt.Println("\nReceived interrupt signal. Exiting chat.")
			return
		}
	}
}

func extractSQL(response string) (string, error) {
	re := regexp.MustCompile("(?s)```sql(.*?)```")
	matches := re.FindStringSubmatch(response)

	if len(matches) > 1 {
		sql := strings.TrimSpace(matches[1])
		return sql, nil
	}

	return strings.TrimSpace(response), nil
}

func connectToDB(ms pkg.Manuscript) (*pgxpool.Pool, error) {
	dbUrl := fmt.Sprintf("postgres://%s:%s@localhost:%d/%s", ms.DbUser, ms.DbPassword, ms.DbPort, ms.Database)
	pool, err := pgxpool.Connect(context.Background(), dbUrl)
	if err != nil {
		return nil, err
	}
	return pool, nil
}

func getTableInfo(pool *pgxpool.Pool) string {
	query := `SELECT table_name, column_name, data_type 
              FROM information_schema.columns 
              WHERE table_schema = 'public';`
	rows, err := pool.Query(context.Background(), query)
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()

	var builder strings.Builder

	for rows.Next() {
		var tableName, columnName, dataType string
		err := rows.Scan(&tableName, &columnName, &dataType)
		if err != nil {
			log.Fatal(err)
		}
		builder.WriteString(fmt.Sprintf("Table: %s, Column: %s, Type: %s\n", tableName, columnName, dataType))
	}

	return builder.String()
}

func executeSQL(pool *pgxpool.Pool, sqlQuery string) {
	rows, err := pool.Query(context.Background(), sqlQuery)
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()

	columns := rows.FieldDescriptions()
	for _, col := range columns {
		fmt.Printf("%s\t", col.Name)
	}
	fmt.Println()

	for rows.Next() {
		values, _ := rows.Values()
		for _, val := range values {
			fmt.Printf("%v\t", val)
		}
		fmt.Println()
	}
	fmt.Println("Do you have any other questions? Type 'exit' to quit.")
}
