package main

import (
  "log"
  "net/http"
  "os"
)

func main() {
  http.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("ok"))
  })

  port := os.Getenv("PORT")
  if port == "" { port = "8080" }
  log.Println("listening on :" + port)
  log.Fatal(http.ListenAndServe(":"+port, nil))
}
