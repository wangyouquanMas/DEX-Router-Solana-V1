import {add} from '../src/add'

describe('add()',() => {
    test('add two positive integers', () =>{
        console.log("result is:",add(2,3))
        expect(add(2,3)).toBe(5)
    })
})