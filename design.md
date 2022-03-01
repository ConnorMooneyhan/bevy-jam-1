# Bevy Jam 1 Game Design Document

## Short Description
A simple maze game with a twist: it's pitch dark the whole time except for when you activate your torch, which can only occur three times.

## Story
You and your little brother snuck out after everyone was asleep because your mom wouldn't let you go through the hay maze earlier. When you get there, you decide to take a quick restroom break first, but while you're gone, your little brother goes ahead and runs through the maze, getting himself lost. You go on a mission to find him, but it's pitch dark out and you can't see a thing! Luckily, when you enter the maze, you meet a strange man who gives you a torch that he says will give you the ability to see the entire maze... but only three times, and it doesn't last long. Make sure your sound is on! You know that by this point your mom must be on her way to come find you. Can you find your little brother before your mom discovers that you've lost him?

## Basic Game Loops
1. Enter the maze
1. Use torch for guidance
1. Navigate maze
1. Save your brother

## Minimum Viable Product (MVP) with time estimates
* Create a maze with "#" symbols for the walls --> 2 hours
* Spawn player marked by "@" who can walk one square at a time --> 30 minutes
* Implement collisions between the player and the walls -- with sound! --> 1 hour
* Place little brother near end of maze --> 15 minutes
* Player wins upon reaching little brother --> 30 minutes
* Wield torch (lasting only 3 uses for a set amount of time each) --> 15 minutes
* Darken maze when torch not in use; lighten when in use --> 1 hour
* Add a timer --> 15 minutes

## Stretch Goals
* Add a torch counter
* Add start and end menus
* Art for player
* Art for walls
* Art for little brother
* Art for ground
* Create beginning interaction sequence with strange man