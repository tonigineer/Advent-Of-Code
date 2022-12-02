% --- Day 8: Space Image Format ---
% 
% The Elves' spirits are lifted when they realize you have an opportunity to 
% reboot one of their Mars rovers, and so they are curious if you would spend 
% a brief sojourn on Mars. You land your ship near the rover.
% 
% When you reach the rover, you discover that it's already in the process of 
% rebooting! It's just waiting for someone to enter a BIOS password. The Elf 
% responsible for the rover takes a picture of the password (your puzzle 
% input) and sends it to you via the Digital Sending Network.
% 
% Unfortunately, images sent via the Digital Sending Network aren't encoded 
% with any normal encoding; instead, they're encoded in a special Space Image 
% Format. None of the Elves seem to remember why this is the case. They send 
% you the instructions to decode it.
% 
% Images are sent as a series of digits that each represent the color of a 
% single pixel. The digits fill each row of the image left-to-right, then 
% move downward to the next row, filling rows top-to-bottom until every pixel 
% of the image is filled.
% 
% Each image actually consists of a series of identically-sized layers that 
% are filled in this way. So, the first digit corresponds to the top-left 
% pixel of the first layer, the second digit corresponds to the pixel to the 
% right of that on the same layer, and so on until the last digit, which 
% corresponds to the bottom-right pixel of the last layer.
% 
% For example, given an image 3 pixels wide and 2 pixels tall, the image data 
% 123456789012 corresponds to the following image layers:
% 
% Layer 1: 123
%          456
% 
% Layer 2: 789
%          012
% 
% The image you received is 25 pixels wide and 6 pixels tall.
% 
% To make sure the image wasn't corrupted during transmission, the Elves 
% would like you to find the layer that contains the fewest 0 digits. On that 
% layer, what is the number of 1 digits multiplied by the number of 2 digits?
% 
% To begin, get your puzzle input.
clear, clc;

fmt = repmat('%s',1,1);
fid = fopen('image_input.txt', 'rt');
img = textscan(fid, fmt, 'Delimiter', '');
img = img{1};
img_str = img{:};
fclose(fid);

% Initialize
width = 25; 
hight = 6;

image_length = width*hight;
num_layer = numel(img_str)/(width*hight);

i = 1;
layer_decode = zeros(num_layer, 4);

% Loop over string 
for layer=1:num_layer
    layer_decode(layer,1) = numel(strfind(img_str(i:i-1+image_length),'0'));
    layer_decode(layer,2) = numel(strfind(img_str(i:i-1+image_length),'1'));
    layer_decode(layer,3) = numel(strfind(img_str(i:i-1+image_length),'2'));
    layer_decode(layer,4) = layer_decode(layer,2)*layer_decode(layer,3);
    i = i + image_length;
end

[~, idx] = min(layer_decode(:,1));
fprintf(['Result: ' num2str(layer_decode(idx, 4)) '\n']);

% Your puzzle answer was 1548.
