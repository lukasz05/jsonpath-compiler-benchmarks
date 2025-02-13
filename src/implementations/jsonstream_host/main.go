package main

import (
	"bytes"
	"io"
	"log"
	"net"
	"os"
	"strings"

	"github.com/arnodel/jsonstream"
	"github.com/arnodel/jsonstream/internal/jsonpath"
	"github.com/arnodel/jsonstream/jsonpathtransformer"
	"github.com/arnodel/jsonstream/token"
)

var jsonInput []byte
var compiledQuery jsonpathtransformer.MainQueryRunner

func main() {
	socket_file := "/tmp/jsonstream_host.sock"
	os.Remove(socket_file)
	socket, err := net.Listen("unix", socket_file)
	if err != nil {
		log.Fatal(err)
	}

	for {
		log.Println("Waiting for an incoming connection...")
		conn, err := socket.Accept()
		if err != nil {
			log.Fatal(err)
		}
		log.Println("Connection accepted.")
		log.Println("Reading data from the connection...")
		buf := make([]byte, 4096)
		n, err := conn.Read(buf)
		if err != nil {
			log.Fatal(err)
		}
		log.Printf("Bytes read: %d.", n)
		if handleMessage(string(buf[:n])) {
			conn.Write([]byte{0})
		} else {
			conn.Write([]byte{1})
		}
		log.Println("Closing the connection...")
		conn.Close()
		log.Println("Connection closed.")
	}
}

func handleMessage(msg string) bool {
	msg = strings.TrimSpace(msg)
	log.Printf("Handling message: '%s'.", msg)
	cmd, arg, _ := strings.Cut(msg, " ")
	switch cmd {
	case "load":
		return loadJsonFile(arg)
	case "compile":
		return compileQuery(arg)
	case "execute":
		return executeQuery()
	default:
		log.Println("Invalid command.")
		return false
	}
}

func loadJsonFile(path string) bool {
	log.Printf("Loading file '%s'...", path)
	var err error
	jsonInput, err = os.ReadFile(path)
	if err != nil {
		log.Printf("Unable to load the file: '%v'.", err)
		return false
	}
	log.Println("File loaded.")
	return true
}

func compileQuery(query string) bool {
	log.Printf("Compiling query '%s'...", query)
	parsedQuery, err := jsonpath.ParseQueryString(query)
	if err != nil {
		log.Printf("Unable to parse the query: '%v'.", err)
		return false
	}
	compiledQuery, err = jsonpathtransformer.CompileQuery(parsedQuery)
	if err != nil {
		log.Printf("Unable to compile the query: '%v'.", err)
		return false
	}
	log.Println("Query compiled.")
	return true
}

func executeQuery() bool {
	log.Println("Executing query...")
	inputStream := bytes.NewReader(jsonInput)
	decoder := jsonstream.NewJSONDecoder(inputStream)
	jsonStream := token.StartStream(
		decoder,
		func(err error) {
			log.Printf("Error while parsing the JSON document: '%v'.", err)
		},
	)
	outputStream := token.TransformStream(jsonStream, compiledQuery)
	encoder := jsonstream.JSONEncoder{
		Printer: &jsonstream.DefaultPrinter{
			Writer: io.Discard,
		},
	}
	err := token.ConsumeStream(outputStream, &encoder)
	if err != nil {
		log.Printf("Error while executing the query: '%v'.", err)
		return false
	}
	return true
}
