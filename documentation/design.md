# ![White king](assets/wK.png) Chess Engine & GUI

## Introduction

The goal of this project is to develop a robust chess engine that implements basic [computer chess](https://www.chessprogramming.org) ideas and strategies.

In addition, we will develop a graphical user interface (GUI) to facilitate interaction with the engine, allowing us to play games and assist with testing during development. The primary purpose of the GUI is to streamline the development process, making it easier to test, debug, and optimize the engine's performance.

## Gameplay

This section describes how the different UI element interact and behave while playing a game of chess.

### Piece interaction

While a player is on their turn, they can chose how to move their pieces in two different ways.

- Dragging
- Point and click

Although the methods are different, the piece UI elements share the same states activated through different actions. The states and how the board behaves depending on the currently active state can be described as follows.

- Piece
  - A piece can be **selected** or **unselected**.
  - At the beginning of every turn, all pieces are unselected.
  - All the player's pieces are selectable during their turn, regardless of whether the piece has any legal moves available.
  - Only once piece can be selected at a time.
  - If a piece is selected, selecting another will automatically unselect the previously selected piece and then select the new one.
  - Attempting to move a piece to its original square will keep the piece selected.
  - Attempting to move a piece to an illegal square or outside the board will unselect the piece.
- Board
  - When a piece is selected, an indicator will appear on all the squares where the piece can legally move.
  - Unselecting a piece will remove all indicators.
  - While a piece is selected, hovering the mouse over squares with an indicator will modify the appearance of that indicator.

Todo.

|                     | Grab   | Drop     | Left click | Right click |
| ------------------- | :----- | :------- | :--------- | :---------- |
| **Dragging**        | select | unselect | -          | -           |
| **Point and click** | -      | -        | select     | unselect    |
