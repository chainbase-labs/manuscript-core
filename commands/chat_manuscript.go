package commands

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"github.com/jackc/pgx/v4/pgxpool"
	"io/ioutil"
	"log"
	"manuscript-core/pkg"
	"net/http"
	"os"
	"os/signal"
	"regexp"
	"strings"
	"syscall"
)

const (
	openaiAPIURL = "https://api.openai.com/v1/chat/completions"
	openaiMode   = "gpt-4o-mini"
)

type ChatGPTRequest struct {
	Model    string          `json:"model"`
	Messages []ChatGPTPrompt `json:"messages"`
}

type ChatGPTPrompt struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type ChatGPTResponse struct {
	Choices []struct {
		Message struct {
			Role    string `json:"role"`
			Content string `json:"content"`
		} `json:"message"`
	} `json:"choices"`
}

type TextToSQLRequest struct {
	Question string `json:"question"`
}

type TextToSQLResponse struct {
	Sql string `json:"sql"`
}

func Chat(manuscript string) {
	jobName := fmt.Sprintf("%s-postgres-1", manuscript)
	dockers, err := pkg.RunDockerPs()
	if err != nil {
		log.Fatalf("Error: Failed to get postgres ps: %v", err)
	}
	if len(dockers) == 0 {
		log.Fatalf("Error: No manuscript postgres found")
	}
	for _, d := range dockers {
		if d.Name == jobName {
			manuscripts, err := pkg.LoadConfig(manuscriptConfig)
			if err != nil {
				log.Fatalf("Error: Failed to load manuscript config: %v", err)
			}
			for _, m := range manuscripts.Manuscripts {
				if m.Name == manuscript {
					ChatWithLLM(m)
				}
			}
			break
		}
	}
}

func ChatWithLLM(job pkg.Manuscript) {
	apiKey := os.Getenv("OPENAI_API_KEY")
	if apiKey == "" {
		log.Fatalf("Error: Please set the environment variable OPENAI_API_KEY")
	}
	model := os.Getenv("OPENAI_MODEL")
	if model == "" {
		model = openaiMode
	}

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
			fmt.Print("You: ")
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

			response, err := sendToChatGPT(prompt, apiKey, model)
			if err != nil {
				fmt.Printf("Error getting response: %v\n", err)
				continue
			}

			fmt.Printf("AI: %s\n", response)

			sqlQuery, err := extractSQL(response)
			if err != nil {
				fmt.Printf("Error extracting SQL: %v\n", err)
				continue
			}
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

func sendToChatGPT(prompt string, apiKey, model string) (string, error) {
	requestData := ChatGPTRequest{
		Model: model,
		Messages: []ChatGPTPrompt{
			{
				Role:    "system",
				Content: "You are a helpful assistant.",
			},
			{
				Role:    "user",
				Content: prompt,
			},
		},
	}

	requestBody, err := json.Marshal(requestData)
	if err != nil {
		return "", fmt.Errorf("error encoding request: %v", err)
	}

	req, err := http.NewRequest("POST", openaiAPIURL, bytes.NewBuffer(requestBody))
	if err != nil {
		return "", fmt.Errorf("error creating request: %v", err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("error sending request: %v", err)
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("error reading response: %v", err)
	}

	var chatGPTResponse ChatGPTResponse
	err = json.Unmarshal(body, &chatGPTResponse)
	if err != nil {
		return "", fmt.Errorf("error decoding response: %v", err)
	}

	if len(chatGPTResponse.Choices) > 0 {
		return chatGPTResponse.Choices[0].Message.Content, nil
	}

	return "", fmt.Errorf("no valid response from ChatGPT")
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
}
