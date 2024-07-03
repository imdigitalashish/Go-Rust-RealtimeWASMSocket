package main

import (
	"log"
	"net/http"

	"go_chat_app/internal/server"

	"golang.org/x/net/websocket"
)

func main() {
	s := server.NewServer()

	http.Handle("/ws", websocket.Handler(s.HandleWS))
	http.Handle("/orderbookfeed", websocket.Handler(s.HandleWSOrderBook))

	log.Println("Server starting on :3000")
	if err := http.ListenAndServe(":3000", nil); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
