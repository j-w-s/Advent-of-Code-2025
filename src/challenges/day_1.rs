/*
--- Day 1: Secret Entrance ---

The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency.
For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency:
according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in.
A document taped to the wall helpfully explains:

"Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order.
As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe.
A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19.
After that, a rotation of L19 would cause it to point at 0.

Because the dial is a circle, turning the dial left from 0 one click makes it point at 99.
Similarly, turning the dial right from 99 one click makes it point at 0.

So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95.
After that, a rotation of R5 could cause it to point at 0.

The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy.
The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

For example, suppose the attached document contained the following rotations:
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82

Following these rotations would cause the dial to move as follows:
    The dial starts by pointing at 50.
    The dial is rotated L68 to point at 82.
    The dial is rotated L30 to point at 52.
    The dial is rotated R48 to point at 0.
    The dial is rotated L5 to point at 95.
    The dial is rotated R60 to point at 55.
    The dial is rotated L55 to point at 0.
    The dial is rotated L1 to point at 99.
    The dial is rotated L99 to point at 0.
    The dial is rotated R14 to point at 14.
    The dial is rotated L82 to point at 32.

Because the dial points at 0 a total of three times during this process, the password in this example is 3.
Analyze the rotations in your attached document. What's the actual password to open the door?
*/
use std::str::Lines;

// 0 => LR68 = (99-68) = 31 => L30 = (31-30) = 1 => R48 = (1+48) = 49 => ... etc
// note: read this as 0, not 50, but i'm too lazy to change it. it stands
// so we need to read in the file and just perform the necessary operations, wrapping
// around range [0,99] via mod

struct Dial {
    pointer: i16,
}

trait Turn {
    fn left(&mut self, distance: i16);
    fn right(&mut self, distance: i16);
    fn process(&mut self, input: &str) -> i16;
}

impl Turn for Dial {
    fn left(&mut self, distance: i16) {
        let distance = distance % 100;
        self.pointer = ((self.pointer - distance) % 100 + 100) % 100;
    }

    fn right(&mut self, distance: i16) {
        let distance = distance % 100;
        self.pointer = (self.pointer + distance) % 100;
    }

    fn process(&mut self, input: &str) -> i16 {
        let mut zero_count = 0;

        for rotation_line in input.lines() {
            let rotation_line = rotation_line.trim();
            if rotation_line.is_empty() {
                continue;
            }

            // parse the rotation command (direction and distance)
            let direction = rotation_line.chars().next().unwrap();
            let distance: i16 = rotation_line[1..].parse().unwrap();

            match direction {
                'L' => self.left(distance),
                'R' => self.right(distance),
                _ => panic!("womp womp: {}", direction),
            }

            println!("after {}: pointer at: {}", rotation_line, self.pointer);

            if self.pointer == 0 {
                // we struck gold buddy 0 has been lapsed
                zero_count += 1;
            }
        }

        zero_count
    }
}

const INPUT: &str = include_str!("day_1.txt");

fn main() {
    let mut dial = Dial { pointer: 50 };
    let password = dial.process(INPUT);
    println!("\npassword is... {}!", password);
}