package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

const (
	chatGPTModel  = "gpt-4o-mini"
	OpenaiApiBase = "https://api.openai.com/v1/chat/completions"
	GaiaApiBase   = "https://llama.us.gaianet.network/v1/chat/completions"
)

type ChatGPTClient struct {
	Name    string
	BaseURL string
	APIKey  string
	Model   string
}

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

func (c *ChatGPTClient) GPTName() string {
	return c.Name
}

func (c *ChatGPTClient) SendRequest(prompt string) (string, error) {
	requestData := ChatGPTRequest{
		Model: c.Model,
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

	if c.BaseURL == "" {
		c.BaseURL = OpenaiApiBase
	}
	if c.Model == "" {
		c.Model = chatGPTModel
	}
	req, err := http.NewRequest("POST", c.BaseURL, bytes.NewBuffer(requestBody))
	if err != nil {
		return "", fmt.Errorf("error creating request: %v", err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+c.APIKey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("error sending request: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
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

	fmt.Println(chatGPTResponse)

	return "", fmt.Errorf("no valid response from %s", c.Name)
}
