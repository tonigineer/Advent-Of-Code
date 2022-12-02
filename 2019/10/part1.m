% --- Day 10: Monitoring Station ---
% 
% You fly into the asteroid belt and reach the Ceres monitoring station. The 
% Elves here have an emergency: they're having trouble tracking all of the 
% asteroids and can't be sure they're safe.
% 
% The Elves would like to build a new monitoring station in a nearby area of 
% space; they hand you a map of all of the asteroids in that region (your 
% puzzle input).
% 
% The map indicates whether each position is empty (.) or contains an 
% asteroid (#). The asteroids are much smaller than they appear on the map, 
% and every asteroid is exactly in the center of its marked position. The 
% asteroids can be described with X,Y coordinates where X is the distance 
% from the left edge and Y is the distance from the top edge (so the top-left 
% corner is 0,0 and the position immediately to its right is 1,0).
% 
% Your job is to figure out which asteroid would be the best place to build 
% a new monitoring station. A monitoring station can detect any asteroid to 
% which it has direct line of sight - that is, there cannot be another 
% asteroid exactly between them. This line of sight can be at any angle, not 
% just lines aligned to the grid or diagonally. The best location is the 
% asteroid that can detect the largest number of other asteroids.
% 
% For example, consider the following map:
% 
% .#..#
% .....
% #####
% ....#
% ...##
% 
% The best location for a new monitoring station on this map is the 
% highlighted asteroid at 3,4 because it can detect 8 asteroids, more than 
% any other location. (The only asteroid it cannot detect is the one at 1,0; 
% its view of this asteroid is blocked by the asteroid at 2,2.) All other 
% asteroids are worse locations; they can detect 7 or fewer other asteroids. 
% Here is the number of other asteroids a monitoring station on each asteroid 
% could detect:
% 
% .7..7
% .....
% 67775
% ....7
% ...87
% 
% Here is an asteroid (#) and some examples of the ways its line of sight 
% might be blocked. If there were another asteroid at the location of a 
% capital letter, the locations marked with the corresponding lowercase 
% letter would be blocked and could not be detected:
% 
% #.........
% ...A......
% ...B..a...
% .EDCG....a
% ..F.c.b...
% .....c....
% ..efd.c.gb
% .......c..
% ....f...c.
% ...e..d..c
% 
% Here are some larger examples:
% 
%     Best is 5,8 with 33 other asteroids detected:
% 
%     ......#.#.
%     #..#.#....
%     ..#######.
%     .#.#.###..
%     .#..#.....
%     ..#....#.#
%     #..#....#.
%     .##.#..###
%     ##...#..#.
%     .#....####
% 
%     Best is 1,2 with 35 other asteroids detected:
% 
%     #.#...#.#.
%     .###....#.
%     .#....#...
%     ##.#.#.#.#
%     ....#.#.#.
%     .##..###.#
%     ..#...##..
%     ..##....##
%     ......#...
%     .####.###.
% 
%     Best is 6,3 with 41 other asteroids detected:
% 
%     .#..#..###
%     ####.###.#
%     ....###.#.
%     ..###.##.#
%     ##.##.#.#.
%     ....###..#
%     ..#.#..#.#
%     #..#.#.###
%     .##...##.#
%     .....#.#..
% 
%     Best is 11,13 with 210 other asteroids detected:
% 
%     .#..##.###...#######
%     ##.############..##.
%     .#.######.########.#
%     .###.#######.####.#.
%     #####.##.#.##.###.##
%     ..#####..#.#########
%     ####################
%     #.####....###.#.#.##
%     ##.#################
%     #####.##.###..####..
%     ..######..##.#######
%     ####.##.####...##..#
%     .#####..#.######.###
%     ##...#.##########...
%     #.##########.#######
%     .####.#.###.###.#.##
%     ....##.##.###..#####
%     .#.#.###########.###
%     #.#.#.#####.####.###
%     ###.##.####.##.#..##
% 
% Find the best location for a new monitoring station. How many other 
% asteroids can be detected from that location?
% 
% To begin, get your puzzle input.
clear, clc;

filename = 'asteroid_map.txt';
d = importdata(filename);

% Get positions of asteroids
positions = [];
for row=1:numel(d)
    dd = d{row};
    for col=1:numel(dd)
        if strcmp(dd(col), '#')
            positions(end+1, 1:2) = [col row];
        end
    end
end

% Loop over all asteroids
for ii=1:numel(positions(:,1))
    headings = [];
    origin(1:2) = positions(ii,1:2);
    % get the headings to every other planet
    for i=1:numel(positions(:,1))
        if all(origin == positions(i,1:2))
            continue
        end
        Y = diff([origin(2) positions(i,2)]);
        X = diff([origin(1) positions(i,1)]);
        headings(end+1) = mod(atan2(Y, X), pi*2);
    end
    positions(ii,3) = numel(unique(headings));
end

[~, idx] = max(positions(:,3));
positions(idx,:)

% Your puzzle answer was 296.
