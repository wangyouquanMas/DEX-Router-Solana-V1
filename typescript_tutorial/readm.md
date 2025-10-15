Purpose: Understand how to create an environment for TypeScript to do tests 

What do we need?

1. Essential Files (REQUIRED)
   
   a) package.json ✅ REQUIRED
      - Manages dependencies (typescript, mocha, chai, etc.)
      - Defines npm scripts (npm test, npm run test:describe)
      - Lists what packages to install
      
   b) tsconfig.json ✅ REQUIRED
      - Configures TypeScript compiler
      - Tells TypeScript how to handle .ts files
      - Sets language features and type checking rules
      
   c) .gitignore ⚠️ RECOMMENDED (optional)
      - Prevents node_modules/ from being committed to Git
      - Keeps repository clean

2. How to Set Up

   Step 1: Create package.json
   ```bash
   npm init -y
   ```
   
   Step 2: Install dependencies
   ```bash
   npm install --save-dev typescript ts-mocha mocha chai @types/mocha @types/chai
   ```
   
   Step 3: Create tsconfig.json
   ```bash
   npx tsc --init
   ```
   Or manually create it with minimal config
   
   Step 4: Write your tests (.test.ts files)
   
   Step 5: Run tests
   ```bash
   npm test
   ```

3. Summary

   YES! You need at minimum:
   - package.json (manages dependencies)
   - tsconfig.json (configures TypeScript)
   
   These 2 files create a complete TypeScript testing environment!
   
See MINIMAL_EXAMPLE.md for detailed examples.
    