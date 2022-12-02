% --- Day 6: Universal Orbit Map ---
% 
% You've landed at the Universal Orbit Map facility on Mercury. Because 
% navigation in space often involves transferring between orbits, the orbit 
% maps here are useful for finding efficient routes between, for example, 
%     you and Santa. You download a map of the local orbits (your puzzle 
%     input).
% 
% Except for the universal Center of Mass (COM), every object in space is 
% in orbit around exactly one other object. An orbit looks roughly like this:
% 
%                   \
%                    \
%                     |
%                     |
% AAA--> o            o <--BBB
%                     |
%                     |
%                    /
%                   /
% 
% In this diagram, the object BBB is in orbit around AAA. The path that BBB 
% takes around AAA (drawn with lines) is only partly shown. In the map data, 
% this orbital relationship is written AAA)BBB, which means "BBB is in orbit 
% around AAA".
% 
% Before you use your map data to plot a course, you need to make sure it 
% wasn't corrupted during the download. To verify maps, the Universal Orbit 
% Map facility uses orbit count checksums - the total number of direct orbits 
% (like the one shown above) and indirect orbits.
% 
% Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain 
% can be any number of objects long: if A orbits B, B orbits C, and C 
% orbits D, then A indirectly orbits D.
% 
% For example, suppose you have the following map:
% 
% COM)B
% B)C
% C)D
% D)E
% E)F
% B)G
% G)H
% D)I
% E)J
% J)K
% K)L
% 
% Visually, the above map of orbits looks like this:
% 
%         G - H       J - K - L
%        /           /
% COM - B - C - D - E - F
%                \
%                 I
% 
% In this visual representation, when two objects are connected by a line, 
% the one on the right directly orbits the one on the left.
% 
% Here, we can count the total number of orbits as follows:
% 
%     D directly orbits C and indirectly orbits B and COM, a total of 3 
%     orbits.
%     L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a 
%     total of 7 orbits.
%     COM orbits nothing.
% 
% The total number of direct and indirect orbits in this example is 42.
% 
% What is the total number of direct and indirect orbits in your map data?
% 
% To begin, get your puzzle input.
% 
% Your puzzle answer was 300598.
% 
% The first half of this puzzle is complete! It provides one gold star: *
% --- Part Two ---
% 
% Now, you just need to figure out how many orbital transfers you (YOU) need 
% to take to get to Santa (SAN).
% 
% You start at the object YOU are orbiting; your destination is the object 
% SAN is orbiting. An orbital transfer lets you move from any object to an 
% object orbiting or orbited by that object.
% 
% For example, suppose you have the following map:
% 
% COM)B
% B)C
% C)D
% D)E
% E)F
% B)G
% G)H
% D)I
% E)J
% J)K
% K)L
% K)YOU
% I)SAN
% 
% Visually, the above map of orbits looks like this:
% 
%                           YOU
%                          /
%         G - H       J - K - L
%        /           /
% COM - B - C - D - E - F
%                \
%                 I - SAN
% 
% In this example, YOU are in orbit around K, and SAN is in orbit around I. 
% To move from K to I, a minimum of 4 orbital transfers are required:
% 
%     K to J
%     J to E
%     E to D
%     D to I
% 
% Afterward, the map of orbits looks like this:
% 
%         G - H       J - K - L
%        /           /
% COM - B - C - D - E - F
%                \
%                 I - SAN
%                  \
%                   YOU
% 
% What is the minimum number of orbital transfers required to move from the 
% object YOU are orbiting to the object SAN is orbiting? (Between the objects 
% they are orbiting - not between YOU and SAN.)
% 
% Although it hasn't changed, you can still get your puzzle input.

clear;
clc;

% // Load map data
fmt = repmat('%s',1,1);
fid = fopen('mapdata.txt', 'rt');
mapdata = textscan(fid, fmt, 'Delimiter', '\n');
mapdata = mapdata{1};
fclose(fid);
p = struct();
% // Bring map data in an ordered struct to analyze it
%   *adding the P is needed for the struct because they cannot start with a
%   digit
for orbit_pair=1:numel(mapdata)
    planets = strsplit(mapdata{orbit_pair},')');
    center = ['P' planets{1}];
    orbit = ['P' planets{2}];
    
%     % replace digit on first position with I -> otherwise you can not
%     % create structs from their names

    p.(orbit).orbits = center;
    p.(center).is_orbited = orbit; 
end

% // Check direct and indirect orbits for every planet
dir_indir_orbits = 0;
planets = {'PYOU'; 'PSAN'};
you_orbits = {};
san_orbits = {};
for planet_number=1:numel(planets)
    planet = planets{planet_number, 1};
    
    while true
        if ~isfield(p.(planet), 'orbits')
            fprintf([planet ' has no orbits\n']);
            break;
        end
        next_planet = p.(planet).orbits;
        
        % count orbit and set planet for another orbit search
        dir_indir_orbits = dir_indir_orbits + 1;
        planet = next_planet;
        
        fprintf([num2str(planet_number) ' ' planet ' ' num2str(dir_indir_orbits) '\n']);
        % reached center of mass, no further orbits possible
        if strcmp(next_planet, 'COM')
            break;
        end
        
        % orbit_lists
        if strcmp('PYOU', planets{planet_number, 1})
            you_orbits{end+1, 1} = next_planet;
        end
        if strcmp('PSAN', planets{planet_number, 1})
            san_orbits{end+1, 1} = next_planet;
        end
        
    end
end

% // determine first cross over
for planet=1:numel(you_orbits)
    if ismember(you_orbits{planet}, san_orbits)
        crossover_planet = you_orbits{planet};
        break;
    end
end


you_num_of_orbits = find(strcmp(you_orbits, crossover_planet));
san_num_of_orbits = find(strcmp(san_orbits, crossover_planet));

shortest_num_of_orbits = you_num_of_orbits+san_num_of_orbits-2

% Your puzzle answer was 520.







