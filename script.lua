io.write("The table the script received has:\n");
print(#foo)
print("ha!")
print(magic)
x = 0
for i = 1, #foo do
	print(i, foo[i])
	x = x + foo[i]
end
io.write("Returning data back to C\n");
return x
