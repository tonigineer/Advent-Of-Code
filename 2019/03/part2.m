% The gravity assist was successful, and you're well on your way to the 
% Venus refuelling station. During the rush back on Earth, the fuel 
% management system wasn't completely installed, so that's next on the 
% priority list.
% 
% Opening the front panel reveals a jumble of wires. Specifically, two 
% wires are connected to a central port and extend outward on a grid. You 
% trace the path each wire takes as it leaves the central port, one wire per 
% line of text (your puzzle input).
% 
% The wires twist and turn, but the two wires occasionally cross paths. To 
% fix the circuit, you need to find the intersection point closest to the 
% central port. Because the wires are on a grid, use the Manhattan distance 
% for this measurement. While the wires do technically cross right at the 
%     central port where they both start, this point does not count, nor 
%     does a wire count as crossing with itself.
% 
% For example, if the first wire's path is R8,U5,L5,D3, then starting from 
% the central port (o), it goes right 8, up 5, left 5, and finally down 3:
% 
% ...........
% ...........
% ...........
% ....+----+.
% ....|....|.
% ....|....|.
% ....|....|.
% .........|.
% .o-------+.
% ...........
% 
% Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, 
% down 4, and left 4:
% 
% ...........
% .+-----+...
% .|.....|...
% .|..+--X-+.
% .|..|..|.|.
% .|.-X--+.|.
% .|..|....|.
% .|.......|.
% .o-------+.
% ...........
% 
% These wires cross at two locations (marked X), but the lower-left one is 
% closer to the central port: its distance is 3 + 3 = 6.
% 
% Here are a few more examples:
% 
%     R75,D30,R83,U83,L12,D49,R71,U7,L72
%     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
%     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
%     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
% 
% What is the Manhattan distance from the central port to the closest 
% intersection?
% =========================================================================
clear; clc;

% // WIRE 1
fmt = repmat('%s',1,1);
fid = fopen('wire1.txt', 'rt');
wire1 = textscan(fid, fmt, 'Delimiter', ',');
wire1 = wire1{1};
fclose(fid);


wire1_coords = zeros(2,numel(wire1)+1);
travel1 = [];
for i = 1:numel(wire1)
    [coord, travel] = navigate(wire1{i}, wire1_coords(:,i));
    wire1_coords(:,i+1) = coord';
    travel1 = [travel1(:,1:end-1) travel];
end

% // WIRE 2
fmt = repmat('%s',1,1);
fid = fopen('wire2.txt', 'rt');
wire2 = textscan(fid, fmt, 'Delimiter', ',');
wire2 = wire2{1};
fclose(fid);

wire2_coords = zeros(2,numel(wire2)+1);
travel2 = [0;0];
for i = 1:numel(wire2)
    [coord, travel] = navigate(wire2{i}, wire2_coords(:,i));
    wire2_coords(:,i+1) = coord';
    travel2 = [travel2(:,1:end-1) travel];
end

% Plotting
figure(1);
hold on; box on; grid on;
plot(wire1_coords(1,:), wire1_coords(2,:),'o');
plot(travel1(1,:), travel1(2,:),'*');
plot(wire2_coords(1,:), wire2_coords(2,:),'o');
plot(travel2(1,:), travel2(2,:),'*');


t1 = travel1';
t2 = travel2';

% Find closest common position
pp = []
[b, idx] = unique(travel1', 'rows');
travel1=travel1(:, idx)
[b, idx] = unique(travel2', 'rows');
travel2=travel2(:,idx)

a = [travel1 travel2]';
[b, idx] = unique(a, 'rows');
c = a;
c(idx,:) = [];

for i=1:numel(c)/2
    if c(i,1) == 0 && c(i,2) == 0
        c(i,:) = inf
    end
end

for i = 1:numel(c)/2
sums(i) = sum(abs(c(i,1:2)));
end
closest_coords = sort(sums);
closest_coords(1)


% // PART 2 ============================================================= /
d = [];
for i=1:numel(c)/2
    if c(i,1)~=inf && c(i,2)~=inf
        d = [d; c(i,:)];
    end
end

% loop over crossings
for cross=1:numel(d)/2
    % steps wire 1
    i = 1;
    while true
        if all(t1(i,:) == d(cross,:))
            steps1=i+1;
            break;
        end
        i=i+1;
    end
    
    % steps wire 2
    i = 1;
    while true
        if all(t2(i,:) == d(cross,:))
            steps2=i+1;
            break;
        end
        i=i+1;
    end
    
    % total
    tot_steps(cross) = steps1+steps2-4;
end

min(tot_steps)

% ========================================================================
% // SUBFUNCTIONS
function [coord, travel] = navigate(cmd, origin)
direction = cmd(1);
vector = str2double(cmd(2:end));
switch direction
    case 'R'
        coord = [origin(1)          origin(2)+vector];
        travel = [[origin(1) ones(1,vector)*origin(1)];                  [origin(2) (1:vector)+origin(2)]];
    case 'L'
        coord = [origin(1)          origin(2)-vector];
        travel = [[origin(1) ones(1,vector)*origin(1)];                  [origin(2) -(1:vector)+origin(2)]];
    case 'U'
        coord = [origin(1)+vector   origin(2)];
        travel = [[origin(1) (1:vector)+origin(1)];                      [origin(2) ones(1,vector)*origin(2)] ];
    case 'D'
        coord = [origin(1)-vector   origin(2)];
        travel = [[origin(1) -(1:vector)+origin(1)];                     [origin(2) ones(1,vector)*origin(2)] ];
    otherwise
        error('not possible');
        
        
end
fprintf(['CMD: ' cmd ' at [' num2str(origin(1)) '/' num2str(origin(2)) '] travels to [' ...
    num2str(coord(1)) '/' num2str(coord(2)) ']\n']);
end



