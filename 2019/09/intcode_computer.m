function [halt, output, opcode, pointer] = intcode_computer(opcode, inputs, pointer)
    % Constants
    ZB = 1;  % opcode is on zero based indexing
    
    % Init 
    halt = false;
    input_counter = 1;
    relative_base = 0;
    
    while pointer <= numel(opcode)
        % Get operation and modes
        [op, m1, m2, m3] = code2opAmodes(opcode, pointer);
        
        % Terminate intcode at 99
        if op == 99
            halt = true;
            output = 0;
            break;
        end
        
        % Command output
%         fprintf(['Sequence: ' num2str(opcode(pointer:pointer+3)') '\n']);
%         fprintf(['OP M1 M2 M3: ' num2str([op, m1, m2, m3]) '\n']);

        % Determine parameters for the operations. Differentiate between
        % position mode (0) and immediate mode (1).
        %
        % ZB is always used in position mode and when the pointer is set
        % from a value (later for 5 and 6).
        if ismember(op, [1 2 7 8])
            if m1 == 2, p1 = relative_base + opcode(pointer+1)+ZB;
                elseif m1 == 1, p1 = pointer+1; 
                else, p1 = opcode(pointer+1)+ZB; 
            end
            if m2 == 2, error('')
                elseif m2, p2 = pointer+2; 
                else, p2 = opcode(pointer+2)+ZB; 
            end
            if m3 == 2, error('')
                elseif m3, p3 = pointer+3; 
                else, p3 = opcode(pointer+3)+ZB; 
            end
        elseif ismember(op, [3 4 9])
            if m1 == 2, p1 = relative_base+opcode(pointer+1)+ZB;
                elseif m1, p1 = pointer+1; 
                else, p1 = opcode(pointer+1)+ZB; 
            end
            p2 = [];
            p3 = [];
        elseif ismember(op,[5 6])
            if m1 == 2, error('')
                elseif m1, p1 = pointer+1; 
                else, p1 = opcode(pointer+1)+ZB; 
            end
            if m2, error('')
                elseif m2, p2 = pointer+2; 
                else, p2 = opcode(pointer+2)+ZB; 
            end
            p3 = [];
        else
            error('op not possible');
        end
        
        % Run operations
        if op == 1
            opcode(p3) = opcode(p1) + opcode(p2);
            pointer = pointer + 4;
        elseif op == 2
            opcode(p3) = opcode(p1) * opcode(p2);
            pointer = pointer + 4;
        elseif op == 3
            opcode(p1) = inputs(input_counter);
            input_counter = input_counter + 1;
            pointer = pointer + 2;
        elseif op == 4
            output = opcode(p1);
            pointer = pointer + 2;
            % checking if next point will be halt just saves one more
            % iteration and the output is right. otherwise next iteration
            % output would be zero at halt.
            if opcode(pointer) == 99
                halt = true;
            end
            break;
        elseif op == 5
            if opcode(p1) ~= 0, pointer = opcode(p2)+ZB; 
            else, pointer = pointer + 3; end
        elseif op == 6
            if opcode(p1) == 0, pointer = opcode(p2)+ZB; 
            else, pointer = pointer + 3; end
        elseif op == 7
            opcode(p3) = opcode(p1)<opcode(p2);
            pointer = pointer + 4;
        elseif op == 8
            opcode(p3) = opcode(p1)==opcode(p2);
            pointer = pointer + 4;
        elseif op == 9
            relative_base = opcode(p1);
            pointer = pointer + 2;
        end
    end
end

% Analyzing opcode to determine to operation and the modes for the
% parameters
function [op, m1, m2, m3] = code2opAmodes(code, pointer)
    op = mod(code(pointer),100);   
    m1 = mod(floor(code(pointer)/100), 10);
    m2 = mod(floor(code(pointer)/1000), 10);
    m3 = mod(floor(code(pointer)/10000), 10);
end