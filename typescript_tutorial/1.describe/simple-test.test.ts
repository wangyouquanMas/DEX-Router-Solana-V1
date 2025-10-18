/**
 * This is a simple test file to demonstrate how 'describe' works
 * 
 * describe() - Groups related tests together
 * it() - Defines individual test cases
 * beforeAll() - Runs once before all tests in a describe block
 * beforeEach() - Runs before each test
 */


//TODO: How to run the specific test file 
// npm run test:describe


//TODO: How to run a nest test ? 
//npx ts-mocha -p ./tsconfig.json 1.describe/simple-test.test.ts --grep "Addition"

// Main test suite
describe("Calculator", () => {
  let result: number;

  // Runs once before all tests in this suite
  beforeAll(() => {
    console.log("Setting up calculator tests...");
  });

  // Nested test suite for Addition
  describe("Addition", () => {
    it("should add two positive numbers", () => {
      result = 2 + 3;
      expect(result).toBe(5);
    });

    it("should add negative numbers", () => {
      result = -5 + (-3);
      expect(result).toBe(-8);
    });

    it("should add zero", () => {
      result = 10 + 0;
      expect(result).toBe(10);
    });
  });

  // Nested test suite for Subtraction
  describe("Subtraction", () => {
    it("should subtract two positive numbers", () => {
      result = 10 - 3;
      expect(result).toBe(7);
    });

    it("should subtract to get negative result", () => {
      result = 5 - 10;
      expect(result).toBe(-5);
    });
  });

  // Nested test suite for Multiplication
  describe("Multiplication", () => {
    it("should multiply two numbers", () => {
      result = 4 * 5;
      expect(result).toBe(20);
    });

    it("should multiply by zero", () => {
      result = 100 * 0;
      expect(result).toBe(0);
    });
  });
});

// Another separate test suite
describe("String Operations", () => {
  
  describe("Concatenation", () => {
    it("should join two strings", () => {
      const result = "Hello" + " " + "World";
      expect(result).toBe("Hello World");
    });
  });

  describe("Length", () => {
    it("should count string length", () => {
      const text = "Solana";
      expect(text.length).toBe(6);
    });
  });
});

