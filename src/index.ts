interface Area {
  area(): number;
}

class Rectangle implements Area {
  constructor(public x: number, public y: number, public width: number, public height: number) {}
  area(): number {
    return this.width * this.height;
  }
  toString() {
    return `Rectangle(${this.x}, ${this.y}): ${this.width}x${this.height}`;
  }
}
class Circle {
  constructor(public x: number, public y: number, public radius: number) {}

  area(): number {
    return this.radius * this.radius * Math.PI;
  }
}

const run = () => {
  console.log(`${new Rectangle(0, 0, 10, 10)}`);
};

run();
// npx ts-node src/index.ts numbers
