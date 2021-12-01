#!/usr/local/bin/fish
function for_part
    echo Creating solution files from template for part $argv[1]
    dump_template ./src/bin/day"$day""$argv[1]".rs
end

function dump_template
    if test -e $argv[1] 
        echo "Solution file $argv[1] already exists; skipping"
        return 
    end
    echo "\
use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!(\"../inputs/input$day.txt\").split_whitespace();
    
    // code here
}
    " > $argv[1]
end

set day $argv[1]
echo Preparing solutions for Advent of Code 2021 Day $day\n
 
for_part a
for_part b

set url https://adventofcode.com/2021/day/$day/input
echo Downloading input file from $url
curl $url > ./src/inputs/input"$day".txt

