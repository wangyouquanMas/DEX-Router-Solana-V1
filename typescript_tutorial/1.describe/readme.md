Goal:
1. What's the meaning of describe ? 
2. How to use it ?


Content:
1. Meaning 
describe is not a part of the TypeScript Language itself but is often associated with testing
frameworks. 

2. Application

2.1 Creates a test suite 
It groups related test cases under a descriptive name 

2.2 Organizes tests hierarchically
You can nest describe blocks for better organization

3. Example

See `simple-test.test.ts` for a complete example showing:
- Main describe block (Calculator)
- Nested describe blocks (Addition, Subtraction, Multiplication)
- Individual test cases using it()
- Setup using before()

4. How to Run (Native TypeScript - No Solana needed!)

**First time setup:**
```bash
cd typescript_tutorial
npm install
```

**Option 1: Using npm script (Recommended)**
```bash
cd typescript_tutorial
npm run test:describe
```

**Option 2: Run all tutorial tests**
```bash
cd typescript_tutorial
npm test
```

**Option 3: Using ts-mocha directly**
```bash
cd typescript_tutorial
npx ts-mocha -p ./tsconfig.json 1.describe/simple-test.test.ts
```

These commands run pure TypeScript/JavaScript tests without needing:
- Solana validator
- Anchor framework
- Blockchain connection
Just pure Mocha + Chai testing!

5. Test Output Structure

When you run the test, you'll see:
```
Calculator
  Setting up calculator tests...
  Addition
    ✓ should add two positive numbers
    ✓ should add negative numbers
    ✓ should add zero
  Subtraction
    ✓ should subtract two positive numbers
    ✓ should subtract to get negative result
  Multiplication
    ✓ should multiply two numbers
    ✓ should multiply by zero

String Operations
  Concatenation
    ✓ should join two strings
  Length
    ✓ should count string length
```

This shows how describe organizes tests hierarchically! 
