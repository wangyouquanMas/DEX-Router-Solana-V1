# Minimal TypeScript Testing Environment

## The 2 Essential Files

### 1. package.json (REQUIRED)

```json
{
  "name": "my-ts-tests",
  "version": "1.0.0",
  "scripts": {
    "test": "ts-mocha -p ./tsconfig.json **/*.test.ts"
  },
  "devDependencies": {
    "@types/chai": "^4.3.11",
    "@types/mocha": "^10.0.6",
    "chai": "^4.4.1",
    "mocha": "^10.2.0",
    "ts-mocha": "^10.0.0",
    "typescript": "^5.3.3"
  }
}
```

**Key parts:**
- `scripts`: Commands you can run with `npm test`
- `devDependencies`: Packages needed for testing

### 2. tsconfig.json (REQUIRED)

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "esModuleInterop": true,
    "strict": true,
    "types": ["mocha", "chai"]
  }
}
```

**Key parts:**
- `target`: What JavaScript version to compile to
- `module`: How to handle imports/exports
- `types`: Tell TypeScript about Mocha and Chai

## Quick Setup Process

```bash
# 1. Create a directory
mkdir my-ts-project
cd my-ts-project

# 2. Create package.json
npm init -y

# 3. Install dependencies
npm install --save-dev typescript ts-mocha mocha chai @types/mocha @types/chai

# 4. Create tsconfig.json
npx tsc --init

# 5. Create your test file (example.test.ts)
# ... write your tests ...

# 6. Run tests
npm test
```

## That's It!

With just these 2 files + your test files, you have a complete TypeScript testing environment! 🚀

## Optional Files (Nice to Have)

- `.gitignore` - Keep Git clean
- `README.md` - Document your project
- `.npmrc` - NPM configuration
- `.editorconfig` - Editor settings

