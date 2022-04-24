q = '0123456789ABCDEF'
for a in q:
    for b in q:
        print(f"({a + b}) => {{ 0x{a + b} }}")