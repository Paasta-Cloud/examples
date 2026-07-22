package main

import (
  "fmt"
  "net/http"
  "os"
)

func main() {
  port := os.Getenv("PORT")
  if port == "" { port = "3000" }
  http.HandleFunc("/", func(w http.ResponseWriter, _ *http.Request) { fmt.Fprint(w, "Paasta Go example") })
  http.ListenAndServe(":"+port, nil)
}
