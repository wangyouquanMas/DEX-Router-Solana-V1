# Vitest Globals Configuration Errors - 学习复盘文档

## 概述
本文档记录了在配置 Vitest 测试框架时遇到的关于全局变量（globals）配置的问题及解决方案。通过这次调试过程，深入理解了 TypeScript 类型定义与运行时配置的区别。

---

## 错误1：Vitest 全局变量未生效

### 背景
在使用 Vitest 进行 TypeScript 项目测试时，已经在 `tsconfig.json` 中配置了 `"types": ["vitest/globals"]`，但测试运行时仍然报错 `ReferenceError: describe is not defined`。

### 错误
```
ReferenceError: describe is not defined
 ❯ tests/add.test.ts:3:1
      3| describe('add()',() => {
       | ^
```

### 原因
1. **类型定义 vs 运行时配置混淆**：`tsconfig.json` 中的 `"types": ["vitest/globals"]` 只是为 TypeScript 提供类型定义，让编辑器能够识别 `describe`、`it`、`expect` 等全局变量的类型，但不会在运行时实际注入这些全局变量。

2. **缺少运行时全局变量启用**：Vitest 需要在运行时明确启用全局变量功能，这需要通过命令行参数 `--globals` 或配置文件来实现。

### 方案
在 `package.json` 的测试脚本中添加 `--globals` 参数：
```json
{
  "scripts": {
    "test": "vitest --globals"
  }
}
```

这告诉 Vitest 在运行时将测试函数作为全局变量注入到测试环境中。

---

## 错误2：Vitest 配置文件模块系统冲突

### 背景
尝试通过创建 `vitest.config.ts` 配置文件来解决全局变量问题，但遇到了 ES 模块与 CommonJS 模块系统的冲突。

### 错误
```
Error [ERR_REQUIRE_ESM]: require() of ES Module ... not supported.
Instead change the require of index.js ... to a dynamic import() which is available in all CommonJS modules.
```

### 原因
1. **模块系统不匹配**：项目配置为 CommonJS 模式（`"module": "commonjs"`），但 Vitest 的配置文件试图使用 ES 模块语法。

2. **依赖版本兼容性问题**：当前版本的 Vitest 与项目的 CommonJS 配置存在兼容性问题。

### 方案
1. **删除配置文件**：移除 `vitest.config.ts` 文件，避免模块系统冲突。
2. **使用命令行参数**：直接在 `package.json` 的测试脚本中通过 `--globals` 参数配置，避免配置文件带来的复杂性。

---

## 错误3：测试框架语法混用

### 背景
在修复全局变量问题后，发现其中一个测试文件使用了 Mocha 风格的语法，而项目使用的是 Vitest（基于 Jest）。

### 错误
```
ReferenceError: before is not defined
 ❯ 1.describe/simple-test.test.ts:25:3
     25| before(() => {
       | ^
```

### 原因
1. **测试框架语法混淆**：测试文件中混用了 Mocha 和 Jest/Vitest 的语法：
   - `before()` 是 Mocha 语法，Vitest 中应该使用 `beforeAll()`
   - `expect().to.equal()` 是 Chai 语法，Vitest 中应该使用 `expect().toBe()`

2. **依赖包不一致**：项目中同时安装了 Chai 和 Vitest，导致测试文件导入了错误的断言库。

### 方案
1. **统一测试语法**：
   - 将 `before()` 改为 `beforeAll()`
   - 将 `expect().to.equal()` 改为 `expect().toBe()`
   - 移除 Chai 的导入语句

2. **清理依赖**：移除不需要的 Chai 相关依赖，统一使用 Vitest 的内置断言。

---

## 关键学习点

1. **TypeScript 类型定义与运行时配置的区别**：类型定义只是为了让 TypeScript 编译器理解代码结构，不会影响运行时行为。

2. **模块系统的兼容性**：在配置工具时需要考虑项目的模块系统设置，避免 ES 模块与 CommonJS 的冲突。

3. **测试框架的一致性**：在一个项目中应该统一使用一种测试框架的语法，避免混用不同框架的特性。

4. **配置的优先级**：命令行参数 > 配置文件 > 默认配置，可以通过简单的命令行参数避免复杂的配置文件设置。

---

## 最终解决方案总结

```json
// package.json
{
  "scripts": {
    "test": "vitest --globals"
  }
}
```

```json
// tsconfig.json
{
  "compilerOptions": {
    "types": ["vitest/globals"]
  }
}
```

通过这两个简单的配置，成功启用了 Vitest 的全局变量功能，测试可以正常运行。
