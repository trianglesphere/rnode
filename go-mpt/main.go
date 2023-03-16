package main

import (
	"fmt"

	// "github.com/davecgh/go-spew/spew"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/ethdb/memorydb"
	"github.com/ethereum/go-ethereum/trie"
)

func main() {
	// ('do', 'verb'), ('dog', 'puppy'), ('doge', 'coin'), ('horse', 'stallion').
	raw := memorydb.New()
	db := trie.NewDatabase(rawdb.NewDatabase(raw))
	t := trie.NewEmpty(db)
	t.Update([]byte("do"), []byte("verb"))
	t.Update([]byte("dog"), []byte("puppy"))
	// t.Update([]byte("doge"), []byte("coin"))
	// t.Update([]byte("horse"), []byte("stallion"))
	hash := t.Hash()
	fmt.Println(db.Node(hash))
	// hash, nodes := t.Commit(false)
	// spew.Dump(nodes)
}
