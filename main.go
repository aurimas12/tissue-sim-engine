package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
)

func fileExists(filename string) bool {
	_, err := os.Stat(filename)
	return err == nil
}

func runCommand(name string, args ...string) {
	cmd := exec.Command(name, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		log.Fatalf("❌ Klaida vykdant komandą: %s %v – %v", name, args, err)
	}
}

func main() {
	fmt.Println("🚀 GAU paleidimas")

	if !fileExists("voxel_data.json") {
		fmt.Println("📄 voxel_data.json nerastas – paleidžiam Python modelio generavimą")
		runCommand("python3", "voxel_model.py")
	}

	fmt.Println("🧠 Paleidžiam Rust engine (difuzija)")
	runCommand("cargo", "run", "--manifest-path", "rust_engine/Cargo.toml")

	if fileExists("result.json") {
		fmt.Println("📊 Paleidžiam Python vizualizaciją")
		runCommand("python3", "visualize_result.py")
	} else {
		log.Fatal("❌ Nerastas result.json – Rust engine gal nepavyko?")
	}
}
