package main

import (
	"fmt"

	"github.com/davecgh/go-spew/spew"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/ethereum/go-ethereum/core/rawdb"
)

func main() {
	// ('do', 'verb'), ('dog', 'puppy'), ('doge', 'coin'), ('horse', 'stallion').
	db := trie.NewDatabase(rawdb.NewMemoryDatabase())
	t := trie.NewEmpty(db)
	t.Update([]byte("do"), []byte("verb"))
	t.Update([]byte("dog"), []byte("puppy"))
	// t.Update([]byte("doge"), []byte("coin"))
	// t.Update([]byte("horse"), []byte("stallion"))
	hash, nodes := t.Commit(true)
	fmt.Println(db.Node(hash))
	fmt.Println(hash)
	spew.Dump(nodes)
}
