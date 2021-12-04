#!/usr/local/bin/fish
# A fish script I use to prepare for a new day's challenges.
# It currently just downloads the newest input file and generates templated
# source files that import the prelude and read the input file.
# For this to work, you need to set the ADVENT_OF_CODE_SESSION environment variable
# to the session ID stored in your cookies.

function for_part
    echo Creating solution files from template for part $argv[1]
    dump_template ./src/bin/day"$day""$argv[1]".rs
end

function dump_template
    if test -e $argv[1] 
        set_color brblack
        echo "Solution file $argv[1] already exists; skipping"
        set_color normal
        return 
    end
    echo "\
use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!(\"../inputs/input$day.txt\").split_terminator('\n');
    
    // code here
}
    " > $argv[1]
end

set day $argv[1]
set_color -o bryellow
echo \>\>\> Preparing solutions for Advent of Code 2021 Day $day
set_color normal
 
for_part a
for_part b

if set -q ADVENT_OF_CODE_SESSION
    set url https://adventofcode.com/2021/day/$day/input
    echo Downloading input file from $url
    curl --cookie "session=$ADVENT_OF_CODE_SESSION" $url > ./src/inputs/input"$day".txt
else
    set_color brred
    echo Cannot download input file: \$ADVENT_OF_CODE_SESSION not set!
    echo Please set it to the session ID found in your cookies!
end

