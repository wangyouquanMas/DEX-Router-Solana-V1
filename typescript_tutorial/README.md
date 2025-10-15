# TypeScript Tutorial

This directory contains TypeScript learning tutorials and examples. All tutorials are self-contained and can be run independently without the main Solana/Anchor project.

## 🚀 Quick Start

### 1. Install Dependencies (First Time Only)

```bash
cd typescript_tutorial
npm install
```

### 2. Run Tests

**Run all tutorial tests:**
```bash
npm test
```

**Run specific tutorial:**
```bash
npm run test:describe
```

## 📚 Tutorials

### 1. describe - Understanding Test Organization

**Location:** `1.describe/`

Learn how to use `describe`, `it`, `before`, and other Mocha testing functions.

**Run:**
```bash
npm run test:describe
```

**Topics covered:**
- What is `describe`?
- How to organize tests hierarchically
- Using `it()` for individual test cases
- Setup with `before()` and `beforeEach()`
- Nested test suites

See `1.describe/readme.md` for detailed documentation.

## 🛠️ Technologies Used

- **TypeScript**: Programming language
- **Mocha**: Testing framework (provides `describe`, `it`)
- **Chai**: Assertion library (provides `expect`)
- **ts-mocha**: Run TypeScript tests directly
- **ts-node**: Execute TypeScript without compilation

## 📝 Adding New Tutorials

1. Create a new directory with a number prefix: `2.topic-name/`
2. Add your tutorial files and tests
3. Update this README
4. Add a test script in `package.json` if needed

## ✅ Benefits

✓ **Standalone**: No Solana or blockchain dependencies  
✓ **Fast**: Tests run in milliseconds  
✓ **Isolated**: Learn TypeScript concepts independently  
✓ **Practical**: Real working examples  

## 📖 Resources

- [Mocha Documentation](https://mochajs.org/)
- [Chai Assertion Library](https://www.chaijs.com/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

