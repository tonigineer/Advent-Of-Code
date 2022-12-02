% --- Part Two ---
% 
% The air conditioner comes online! Its cold air feels good for a while, but 
% then the TEST alarms start to go off. Since the air conditioner can't vent 
% its heat anywhere but back into the spacecraft, it's actually making the 
% air inside the ship warmer.
% 
% Instead, you'll need to use the TEST to extend the thermal radiators. 
% Fortunately, the diagnostic program (your puzzle input) is already equipped 
% for this. Unfortunately, your Intcode computer is not.
% 
% Your computer is only missing a few opcodes:
% 
%     Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets 
%     the instruction pointer to the value from the second parameter. 
%     Otherwise, it does nothing.
%     Opcode 6 is jump-if-false: if the first parameter is zero, it sets the 
%     instruction pointer to the value from the second parameter. Otherwise, 
%     it does nothing.
%     Opcode 7 is less than: if the first parameter is less than the second 
%     parameter, it stores 1 in the position given by the third parameter. 
%     Otherwise, it stores 0.
%     Opcode 8 is equals: if the first parameter is equal to the second 
%     parameter, it stores 1 in the position given by the third parameter. 
%     Otherwise, it stores 0.
% 
% Like all instructions, these instructions need to support parameter modes 
% as described above.
% 
% Normally, after an instruction is finished, the instruction pointer 
% increases by the number of values in that instruction. However, if the 
%     instruction modifies the instruction pointer, that value is used and 
%     the instruction pointer is not automatically increased.
% 
% For example, here are several programs that take one input, compare it to 
% he value 8, and then produce one output:
% 
%     3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the 
%     input is equal to 8; output 1 (if it is) or 0 (if it is not).
%     3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the 
%     input is less than 8; output 1 (if it is) or 0 (if it is not).
%     3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the 
%     input is equal to 8; output 1 (if it is) or 0 (if it is not).
%     3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the 
%     input is less than 8; output 1 (if it is) or 0 (if it is not).
% 
% Here are some jump tests that take an input, then output 0 if the input 
% was zero or 1 if the input was non-zero:
% 
%     3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
%     3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
% 
% Here's a larger example:
% 
% 3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
% 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
% 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
% 
% The above example program uses an input instruction to ask for a single 
% number. The program will then output 999 if the input value is below 8, 
% output 1000 if the input value is equal to 8, or output 1001 if the input 
% value is greater than 8.
% 
% This time, when the TEST diagnostic program runs its input instruction to 
% get the ID of the system to test, provide it 5, the ID for the ship's 
% thermal radiator controller. This diagnostic test suite only outputs one 
% number, the diagnostic code.
% 
% What is the diagnostic code for system ID 5?
% 
% Your puzzle answer was 8684145.

clear; clc;

% // Load map data
fmt = repmat('%s',1,1);
fid = fopen('opcode.txt', 'rt');
opcode = textscan(fid, fmt, 'Delimiter', ',');
opcode = opcode{1};
fclose(fid);

% // Preprocessing
op = zeros(numel(opcode),1);
for i=1:numel(opcode)
    op(i,1) = str2double(opcode{i});
end
opcode = op;
% opcode = vec2mat(opcode,4);  % sequence has always the length of 


inputs = 5;

% // CALCULATE OPCODE ================================================== //
output = intcode_computer(opcode, inputs)

function output = intcode_computer(opcode, inputs)
    % Constants
    ZB = 1; % code is made for zerobased indexing
    
    % Initialize
    css = 1;
    input_number = 1;

    % Run intcode
    while true % command sequence start
    % PREPARATIONS ===================================================== //
        % Terminater found, end code
        if opcode(css) == 99
            fprintf('Terminater (99) found.\n');
            break;
        end

        % Output current sequnce, debugging
        fprintf(['SEQUNECE: ' num2str(opcode(css)) ' ' num2str(opcode(css+1)) ...
            ' ' num2str(opcode(css+2)) ' ' num2str(opcode(css+3)) '\n']);

        % Add leading zero to opcode (first position of sequence). If no
        % leading zero, the STRCMP would not work when when no parameter
        % are given (1104 w param, 4 w/o param)
        oc = num2str(opcode(css));
        if length(oc) == 1
            oc = ['0' oc];
        end

   	% RUN OPCODE ======================================================= //
        % Multiplication
        if strcmp(oc(end-1:end), '01')
            calc_mode = '+';
        elseif strcmp(oc(end-1:end), '02')
            calc_mode = '*';
        elseif strcmp(oc(end-1:end), '03')
            fprintf(['Input used: ' num2str(inputs(input_number)) '\n'])
            opcode(opcode(css+1)+ZB) = inputs(input_number);
            input_number = input_number + 1;
            css = css + 2;
            continue;
        elseif strcmp(oc(end-1:end), '04')
            if length(oc) == 3
                if strcmp(oc(1), '0')
                    output = opcode(opcode(css+1)+ZB);
                else
                    output = opcode(css+1);
                end
            else
                output = opcode(opcode(css+1)+ZB);
            end
            fprintf(['   Output: ' num2str(output) '\n']);
            css = css + 2;
            continue  
        elseif strcmp(oc(end-1:end), '07')
            if length(oc)>=3
                if strcmp(oc(end-2),'0')
                    factor1 = opcode(opcode(css+1)+ZB);
                else
                    factor1 = opcode(css+1);
                end
            else
                factor1 = opcode(opcode(css+1)+ZB);
            end

            if length(oc)>=4
                if strcmp(oc(end-3),'0')
                    factor2 = opcode(opcode(css+2)+ZB);
                else
                    factor2 = opcode(css+2);
                end
            else
                factor2 = opcode(opcode(css+2)+ZB);
            end
            opcode(opcode(css+3)+ZB) = factor1<factor2;
            css = css + 4;
            continue
        elseif strcmp(oc(end-1:end), '08')
            if length(oc)>=3
                if strcmp(oc(end-2),'0')
                    factor1 = opcode(opcode(css+1)+ZB);
                else
                    factor1 = opcode(css+1);
                end
            else
                factor1 = opcode(opcode(css+1)+ZB);
            end

            if length(oc)>=4
                if strcmp(oc(end-3),'0')
                    factor2 = opcode(opcode(css+2)+ZB);
                else
                    factor2 = opcode(css+2);
                end
            else
                factor2 = opcode(opcode(css+2)+ZB);
            end
            opcode(opcode(css+3)+ZB) = factor1==factor2;
            css = css + 4;
            continue
        elseif strcmp(oc(end-1:end), '05')
            if length(oc)>=3
                if strcmp(oc(end-2), '0')
                    val = opcode(opcode(css+1)+ZB);
                else
                    val = opcode(css+1);
                end
            else
                val = opcode(opcode(css+1)+ZB);
            end
            % check if non zero
            if val ~= 0
                if length(oc)>=4
                   if strcmp(oc(end-3), '0')
                      css = opcode(opcode(css+2)+ZB)+ZB;
                   else
                       css = opcode(css+2)+1;
                   end
                else 
                    css = opcode(opcode(css+2)+ZB)+ZB;
                end
               continue
            end
            css = css + 3;
            continue
        elseif strcmp(oc(end-1:end), '06')
            if length(oc)>=3
                if strcmp(oc(end-2), '0')
                    val = opcode(opcode(css+1)+ZB);
                else
                    val = opcode(css+1);
                end
            else
                val = opcode(opcode(css+1)+ZB);
            end
            % check if non zero
            if val == 0
                if length(oc)>=4
                   if strcmp(oc(end-3), '0')
                      css = opcode(opcode(css+2)+ZB)+ZB;
                   else
                       css = opcode(css+2)+1;
                   end
                else 
                    css = opcode(opcode(css+2)+ZB)+ZB;
                end
               continue
            end
            css = css + 3;
            continue


        else
            error('calc mode gibt es nicht');
        end

        % // determine first_mode 
        if length(oc)>=3
            if strcmp(oc(end-2), '0')
                first_mode = 'pos';
            else
                first_mode = 'imm';
            end
        else
            first_mode = 'pos';
        end

        % // determine second_mode
        if length(oc)>=4
            if strcmp(oc(end-3), '0')
                second_mode = 'pos';
            else
                second_mode = 'imm';
            end
        else
            second_mode = 'pos';    
        end

        % // determine store_mode
        if length(oc)>=5
            if strcmp(oc(end-4), '0')
                store_mode = 'pos';
            else
                store_mode = 'imm';
            end
        else
            store_mode = 'pos';
        end
    %         fprintf([calc_mode ' ' first_mode ' ' second_mode ' ' store_mode '\n'])

        % // build command for ADDING AND MULTIPLICATION
        if strcmp(first_mode, 'pos')
            value1 = opcode(opcode(css+1)+ZB);
        else
            value1 = opcode(css+1);
        end
        if strcmp(second_mode, 'pos')
            value2 = opcode(opcode(css+2)+ZB);
        else
            value2 = opcode(css+2);
        end
        if strcmp(store_mode, 'pos')
            target = opcode(css+3)+ZB;
        else
            target = css+3;
        end

        cmd = ['opcode(' num2str(target) ') = ' num2str(value1) ' ' ...
            calc_mode ' ' num2str(value2) ';'];
        fprintf(['   CMD: ' cmd '\n']);
        eval(cmd);      

        % jump to next sequence automatically
        css = css + 4;
    end
end
% Your puzzle answer was 8684145.

