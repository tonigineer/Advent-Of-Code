function opcode = load_opcode(filename)
% Open text file and delimit input
fmt = repmat('%s',1,1);
fid = fopen(filename, 'rt');
opcode = textscan(fid, fmt, 'Delimiter', ',');
opcode = opcode{1};
fclose(fid);

% Convert string data into vector
op = zeros(numel(opcode),1);
for i=1:numel(opcode)
    op(i,1) = str2double(opcode{i});
end
opcode = op;
end