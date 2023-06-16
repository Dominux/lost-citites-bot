# The Lost Cities bot

An attempt to create a bot for the Lost Cities board game

## Idea overview

Since the term [AI](https://en.wikipedia.org/wiki/Artificial_intelligence) refers to any algorithm that uses any sort of emulating intelligent work, game bots are AI algorithms by default since they emulate human players. Doesn't matter what approach is used to implement such algorithm ([ML](https://en.wikipedia.org/wiki/Machine_learning), [DL](https://en.wikipedia.org/wiki/Deep_learning), simple `if-else` hardcoded scripts and strategies or any combination of them structured in any sort of complex system) or how well it performs its job, it's still an AI.

So the idea is to create a bot with hardcoded conditional scripts that should:

1. Watch the game process
2. Memorise cards and their game location
3. Compute probability of beneficial events, such as getting desired cards, and its foe getting undesired ones
4. Influence the game process in such a way to prevent its foe from getting benefits mainly by slowering his progress

**Since the idea of the game is to make it better than your foe by outperforming him by game points, the bot should generally act to maximise its success and to minimize its foe success**

I assume that the game process is simple to crack with just computing probability and make some bad things for the foe. I guess creating conditional strategies manually is enough to create a great bot that will be nearly invincible.

## Repo components

This repo is considered to be self-sufficient. The desired components are:

1. Low-level game logic library
2. High-level gameplay as a webapp
3. Game bot itself

The game will can be played in a single device, without any cross-device interaction (for saving game states, histories, stats or providing a cross-device multiplayer) since I'm not gonna create a server cause its additional work and hosting need (there's no free VDS/VPS providers with unlimited bandwidth) and many other backend troubles (system scalling, supervising and so on). The goal is to create simple playable game with the best bot. Multiplayer will be available in a form of playing on a single devices!
