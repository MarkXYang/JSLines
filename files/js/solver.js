class Sameballs {
    constructor(start, count) {
        this.start = start; // Start position of continuous same color balls
        this.count = count; // How many same color balls
    }
    isfive() {
        return (this.count >= 5) ? true : false;
    }
    reset(start) {
        this.start = start;
        this.count = 0;
    }
    copyIfMoreThan(theone) {
        if(this.count < theone.count) {
            this.count = theone.count;
            this.start = theone.start;
        }
    }
}

class Solver { 
    constructor(grid) {
        this.grid = grid;
        this.score = 0;
    }

    isVerticalLine(x) {
        var sameballs = new Sameballs(0, 0);
        var max_sameballs = new Sameballs(0, 0);
        for(var i = 1; i < this.grid.size; i++) {
            if(typeof(this.grid.data[i][x]) !== "number") {
                if(((this.grid.data[i][x]).equalColour(this.grid.data[i-1][x]))) {
                    sameballs.count += 1;
                } else {
                    max_sameballs.copyIfMoreThan(sameballs);
                    sameballs.reset(i);
                }
            } else {
                max_sameballs.copyIfMoreThan(sameballs);
                sameballs.reset(i);
            }
        }
        max_sameballs.copyIfMoreThan(sameballs);

        // The real count is 1 more
        max_sameballs.count++;
        return max_sameballs;
    }

    clearVertical(x, y, num) {
        for(var i = 0; i < num; i++) {
            this.grid.data[y + i][x] = 0;
        }
    }

    checkVerticalLines() {
        var result = false;
        for(var x = 0; x < this.grid.size; x++) {
            var max = this.isVerticalLine(x);

            if(max.isfive()) {
                this.clearVertical(x, max.start, max.count);
                Utils.increaseScore(max.count);
                result = true;
            }
        }
        return result;
    }

    isHorizontalLine(y) {
        var sameballs = new Sameballs(0, 0);
        var max_sameballs = new Sameballs(0, 0);
        for(var i = 1; i < this.grid.size; i++) {
            if(typeof(this.grid.data[y][i]) !== "number") {
                if(((this.grid.data[y][i]).equalColour(this.grid.data[y][i-1]))) {
                    sameballs.count += 1;
                } else {
                    max_sameballs.copyIfMoreThan(sameballs);
                    sameballs.reset(i);
                }
            } else {
                max_sameballs.copyIfMoreThan(sameballs);
                sameballs.reset(i);
            }
        }
        max_sameballs.copyIfMoreThan(sameballs);

        // The real count is 1 more
        max_sameballs.count++;

        return max_sameballs;
    }

    clearHorizontal(x, y, num) {
        for(var i = 0; i < num; i++) {
            this.grid.data[y][x + i] = 0;
        }
    }

    checkHorizontalLines() {
        var result = false;
        for(var y = 0; y < this.grid.size; y++) {
            var max = this.isHorizontalLine(y);
            if(max.isfive()) {
                this.clearHorizontal(max.start, y, max.count);
                Utils.increaseScore(max.count);
                result = true;
            }
        }
        return result;
    }
    

    isDiagonalLeftLine(x, y) {
        var result = true;
        for(var i = 0; i < 5; i++) {
            if(typeof(this.grid.data[y + i][x - i]) !== "number") {
                if(!(this.grid.data[y + i][x - i].equalColour(this.grid.data[y][x]))) {
                    result = false;
                }
            } else {
                result = false;
            }
        }
        return result;
    }

    clearDiagonalLeftLine(x, y) {
        for(var i = 0; i < 5; i++) {
            this.grid.data[y + i][x - i] = 0;
        }
    }

    checkDiagonalLeftLines() {
        var result = false;
        for(var x = this.grid.size - 1; x > 4; x--) {
            for(var y = 0; y < 5; y++) {
                if(this.isDiagonalLeftLine(x, y)) {
                    this.clearDiagonalLeftLine(x, y);
                    Utils.increaseScore(this.score);
                    result = true;
                }
            }
        }
        return result;
    }

    isDiagonalRightLine(x, y) {
        var result = true;
        for(var i = 0; i < 5; i++) {
            if(typeof(this.grid.data[y + i][x + i]) !== "number") {
                if(!(this.grid.data[y + i][x + i].equalColour(this.grid.data[y][x]))) {
                    result = false;
                }
            } else {
                result = false;
            }
        }
        return result;
    }

    clearDiagonalRightLine(x, y) {
        for(var i = 0; i < 5; i++) {
            this.grid.data[y + i][x + i] = 0;
        }
    }

    checkDiagonalRightLines() {
        var result = false;
        for(var x = 0; x < 5; x++) {
            for(var y = 0; y < 5; y++) {
                if(this.isDiagonalRightLine(x, y)) {
                    this.clearDiagonalRightLine(x, y);
                    Utils.increaseScore(this.score);
                    result = true;
                }
            }
        }
        return result;
    }

    scanLines() {
        return this.checkHorizontalLines() || this.checkVerticalLines() || this.checkDiagonalRightLines() || this.checkDiagonalLeftLines();
    }
}