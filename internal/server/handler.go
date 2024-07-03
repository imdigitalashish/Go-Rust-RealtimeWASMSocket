package server

import (
	"fmt"
	"io"
	"log"
	"time"

	"golang.org/x/net/websocket"
)

func (s *Server) HandleWS(ws *websocket.Conn) {
	log.Printf("New incoming connection from client: %s", ws.RemoteAddr())
	s.conns[ws] = true
	s.readLoop(ws)
}

func (s *Server) HandleWSOrderBook(ws *websocket.Conn) {
	log.Printf("New incoming connection for orderbook feed from client: %s", ws.RemoteAddr())
	for {
		payload := fmt.Sprintf("orderbook data -> %d\n", time.Now().UnixNano())
		if _, err := ws.Write([]byte(payload)); err != nil {
			log.Printf("Error writing to orderbook feed: %v", err)
			return
		}
		time.Sleep(2 * time.Second)
	}
}

func (s *Server) readLoop(ws *websocket.Conn) {
	buf := make([]byte, 1024)
	for {
		n, err := ws.Read(buf)
		if err != nil {
			if err == io.EOF {
				break
			}
			log.Printf("Read error: %v", err)
			continue
		}
		msg := buf[:n]
		s.broadcast(msg)
	}
}
