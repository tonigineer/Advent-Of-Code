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
planets = fieldnames(p);
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
    end
end

dir_indir_orbits

% Your puzzle answer was 300598.




