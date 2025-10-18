Objective: Know how to write a unit test with typescript faster 


Content:
1. Download Vitest
npm install -D vitest

2. How to use it ?
2.1 add a script in package.json
"scripts:{
    "test":"vitest",
}"

2.2 How to specify a unit test to run.
npm run test tests/add.test.ts


3. describe 
purpose: it's used to grup test functions together
grammar: describe(name, fn)

3.1 it 
purpose: it means a single test function 
grammar: it(name,fn)
