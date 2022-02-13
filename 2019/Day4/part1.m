% --- Day 4: Secure Container ---
% 
% You arrive at the Venus fuel depot only to discover it's protected by a 
% password. The Elves had written the password on a sticky note, but someone 
% threw it out.
% 
% However, they do remember a few key facts about the password:
% 
%     It is a six-digit number.
%     The value is within the range given in your puzzle input.
%     Two adjacent digits are the same (like 22 in 122345).
%     Going from left to right, the digits never decrease; they only ever 
%     increase or stay the same (like 111123 or 135679).
% 
% Other than the range rule, the following are true:
% 
%     111111 meets these criteria (double 11, never decreases).
%     223450 does not meet these criteria (decreasing pair of digits 50).
%     123789 does not meet these criteria (no double).
% 
% How many different passwords within the range given in your puzzle input 
% meet these criteria?
% 
% Your puzzle input is 134792-675810.

range = [134792 675810];
passwords = [];

for possible_password=range(1):range(2)
    % convert int to string
    password_digits = str2double(regexp(num2str(possible_password),'\d','match'));
        
    % check adjacent_digits criteria
    adjacent_digits = false;
    for digit=1:5
        if password_digits(digit) == password_digits(digit+1)
          adjacent_digits = true;
          break;
        end
    end
    
    % check never_decrease criteria
    never_decrease = true;
    for digit=1:5
        if password_digits(digit) > password_digits(digit+1)
          adjacent_digits = false;
          break;
        end
    end
    
    % check if criteria were met
    if adjacent_digits == true && never_decrease == true
        passwords = [passwords possible_password];
    end
end

number_of_possible_passwords = numel(passwords)




