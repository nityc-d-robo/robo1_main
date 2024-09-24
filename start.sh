#!/bin/bash

tmux new-session -d -s robo

tmux send-keys -t robo "zsh" C-m
tmux send-keys -t robo "cd robo1_main" C-m
tmux send-keys -t robo "cargo run --release" C-m

