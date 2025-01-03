serve: kill
    cargo leptos watch

release:
    cargo leptos build --release --precompress

docker_release:
    docker build -t scorepad .

kill: 
    -lsof -i :10000 | awk 'NR==2 {print $2}' | xargs kill

zellij:
    zellij --layout zellij-layout.kdl

    
tailwind_watch:
    cd scorepad/style && npx tailwindcss -i input.css -o main.css --watch


dependencies:
    cargo install cargo-watch
    cargo install leptosfmt
    cd scorepad/style && npm install
    rustup target add wasm32-unknown-unknown

fix:
    leptosfmt scorepad
    cargo fmt --all
    cargo fix --bin scorepad --allow-dirty
