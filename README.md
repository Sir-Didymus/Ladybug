<div align="center">
<img src="ladybug.png" width="200" alt="ladybug.pn">
<h3>Ladybug</h3>

A free and <s>strong</s> weak UCI chess engine.
</div>


This is my attempt at writing a chess engine in Rust. 
Before starting this project, I knew next to nothing about neither chess engines nor Rust.

My long term goal for this project is:

- Write a chess engine that can beat me consistently (~1500 Blitz on Chess.com).

My short term goals for version 0.1.0 are:

- Write a move generator that can generate all legal moves for a given position without error and  in reasonable time.
- Write an "AI" that, for now, chooses a random move from all legal moves and plays that move.
This "AI" will of course be improved later on to actually evaluate the position. 
For now, I just want to create a baseline which I can compare future version against.
- Implement the UCI (Universal Chess Interface) protocol so that I can plug my engine into the various chess GUIs out there,
or even create a lichess bot that I and others can play against.