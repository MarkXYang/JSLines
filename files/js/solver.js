/**
 * Represents a sequence of continuous same-colored balls.
 * This class is used by the Solver to track potential lines.
 */
class ContinuousColorBalls {
    /**
     * Creates a new ContinuousColorBalls instance.
     * @param {number} x - The starting x-coordinate (column index) of the sequence.
     * @param {number} y - The starting y-coordinate (row index) of the sequence.
     * @param {number} count - The number of same-colored balls in the sequence.
     */
    constructor(x, y, count) {
        /** @type {number} */
        this.x = x;
        /** @type {number} */
        this.y = y;
        /** @type {number} */
        this.count = count;
    }

    /**
     * Checks if the sequence contains 5 or more balls (meaning it forms a line).
     * @returns {boolean} True if the count is 5 or more, false otherwise.
     */
    isFiveOrMore() { // Renamed from isfive for clarity
        return this.count >= 5;
    }

    /**
     * Resets the sequence with a new starting position and count.
     * Typically used when a sequence is broken or a new one starts.
     * @param {number} x - The new starting x-coordinate.
     * @param {number} y - The new starting y-coordinate.
     */
    reset(x, y) {
        this.x = x;
        this.y = y;
        this.count = 0; // Reset count to 0, as it usually means the current ball is different or empty
    }

    /**
     * Updates this sequence if another sequence (`otherSequence`) is longer.
     * This is used to keep track of the longest sequence found so far.
     * @param {ContinuousColorBalls} otherSequence - The other sequence to compare with.
     */
    updateIfGreater(otherSequence) {
        if (this.count < otherSequence.count) {
            this.count = otherSequence.count;
            this.x = otherSequence.x;
            this.y = otherSequence.y;
        }
    }
}

/**
 * Handles the logic for detecting and clearing lines of balls in the grid.
 */
class Solver {
    /**
     * Creates a new Solver instance.
     * @param {Grid} grid - The game grid to operate on.
     */
    constructor(grid) {
        /** @type {Grid} */
        this.grid = grid;
        /**
         * The current score achieved by clearing lines.
         * @type {number}
         */
        this.score = 0; // This score property seems local to solver, game might have its own score.
    }

    /**
     * Checks for the longest vertical line of same-colored balls in a given column.
     * @param {number} columnIndex - The column index to check.
     * @returns {ContinuousColorBalls} An object representing the longest continuous sequence found.
     *                                 The count is initialized to 1 if any ball is found, as a single ball is a sequence of 1.
     */
    findLongestVerticalSequence(columnIndex) {
        let currentSequence = new ContinuousColorBalls(columnIndex, 0, 0);
        let maxSequence = new ContinuousColorBalls(columnIndex, 0, 0);

        for (let rowIndex = 0; rowIndex < this.grid.size; rowIndex++) {
            const currentCell = this.grid.data[rowIndex][columnIndex];
            if (typeof currentCell === "object") { // Cell contains a ball
                if (currentSequence.count === 0) { // Start of a new potential sequence
                    currentSequence.reset(columnIndex, rowIndex);
                    currentSequence.count = 1;
                } else {
                    const previousCell = this.grid.data[rowIndex - 1][columnIndex];
                    if (typeof previousCell === "object" && currentCell.equalColour(previousCell)) {
                        currentSequence.count++;
                    } else { // Sequence broken
                        maxSequence.updateIfGreater(currentSequence);
                        currentSequence.reset(columnIndex, rowIndex);
                        currentSequence.count = 1;
                    }
                }
            } else { // Cell is empty, sequence broken
                maxSequence.updateIfGreater(currentSequence);
                currentSequence.reset(columnIndex, rowIndex); // Reset with current empty cell's coords, count remains 0
            }
        }
        maxSequence.updateIfGreater(currentSequence); // Final check for the last sequence
        return maxSequence;
    }

    /**
     * Clears a vertical line of balls from the grid.
     * @param {ContinuousColorBalls} lineInfo - Information about the line to clear.
     */
    clearVerticalLine(lineInfo) {
        for (let i = 0; i < lineInfo.count; i++) {
            this.grid.data[lineInfo.y + i][lineInfo.x] = 0; // Set cell to empty
        }
    }

    /**
     * Checks all columns for vertical lines and clears them if found.
     * @returns {boolean} True if any vertical line was found and cleared, false otherwise.
     */
    checkAndClearVerticalLines() {
        let lineFound = false;
        for (let colIndex = 0; colIndex < this.grid.size; colIndex++) {
            const longestLine = this.findLongestVerticalSequence(colIndex);
            if (longestLine.isFiveOrMore()) {
                this.clearVerticalLine(longestLine);
                Utils.increaseScore(longestLine.count);
                this.score += longestLine.count;
                lineFound = true;
            }
        }
        return lineFound;
    }

    /**
     * Checks for the longest horizontal line of same-colored balls in a given row.
     * @param {number} rowIndex - The row index to check.
     * @returns {ContinuousColorBalls} An object representing the longest continuous sequence found.
     */
    findLongestHorizontalSequence(rowIndex) {
        let currentSequence = new ContinuousColorBalls(0, rowIndex, 0);
        let maxSequence = new ContinuousColorBalls(0, rowIndex, 0);

        for (let colIndex = 0; colIndex < this.grid.size; colIndex++) {
            const currentCell = this.grid.data[rowIndex][colIndex];
            if (typeof currentCell === "object") { // Cell contains a ball
                if (currentSequence.count === 0) { // Start of a new potential sequence
                    currentSequence.reset(colIndex, rowIndex);
                    currentSequence.count = 1;
                } else {
                    const previousCell = this.grid.data[rowIndex][colIndex - 1];
                    if (typeof previousCell === "object" && currentCell.equalColour(previousCell)) {
                        currentSequence.count++;
                    } else { // Sequence broken
                        maxSequence.updateIfGreater(currentSequence);
                        currentSequence.reset(colIndex, rowIndex);
                        currentSequence.count = 1;
                    }
                }
            } else { // Cell is empty
                maxSequence.updateIfGreater(currentSequence);
                currentSequence.reset(colIndex, rowIndex);
            }
        }
        maxSequence.updateIfGreater(currentSequence);
        return maxSequence;
    }

    /**
     * Clears a horizontal line of balls from the grid.
     * @param {ContinuousColorBalls} lineInfo - Information about the line to clear.
     */
    clearHorizontalLine(lineInfo) {
        for (let i = 0; i < lineInfo.count; i++) {
            this.grid.data[lineInfo.y][lineInfo.x + i] = 0;
        }
    }

    /**
     * Checks all rows for horizontal lines and clears them if found.
     * @returns {boolean} True if any horizontal line was found and cleared, false otherwise.
     */
    checkAndClearHorizontalLines() {
        let lineFound = false;
        for (let rowIndex = 0; rowIndex < this.grid.size; rowIndex++) {
            const longestLine = this.findLongestHorizontalSequence(rowIndex);
            if (longestLine.isFiveOrMore()) {
                this.clearHorizontalLine(longestLine);
                Utils.increaseScore(longestLine.count);
                this.score += longestLine.count;
                lineFound = true;
            }
        }
        return lineFound;
    }

    /**
     * Finds the longest sequence of same-colored balls along a top-left to bottom-right diagonal.
     * Diagonals are identified by their starting (x, y) coordinates.
     * The length of the diagonal scan is determined by how many steps can be taken before going out of bounds.
     * @param {number} startX - The starting x-coordinate of the diagonal.
     * @param {number} startY - The starting y-coordinate of the diagonal.
     * @returns {ContinuousColorBalls} The longest sequence found on this diagonal.
     */
    findLongestDiagonalRightSequence(startX, startY) {
        let currentSequence = new ContinuousColorBalls(startX, startY, 0);
        let maxSequence = new ContinuousColorBalls(startX, startY, 0);
        // Determine how many steps can be taken along this diagonal before hitting the grid boundary.
        const maxSteps = Math.min(this.grid.size - startX, this.grid.size - startY);

        for (let step = 0; step < maxSteps; step++) {
            const currentX = startX + step;
            const currentY = startY + step;
            const currentCell = this.grid.data[currentY][currentX];

            if (typeof currentCell === "object") { // Cell contains a ball
                if (currentSequence.count === 0) { // New sequence
                    currentSequence.reset(currentX, currentY);
                    currentSequence.count = 1;
                } else {
                    const previousCell = this.grid.data[currentY - 1][currentX - 1];
                    if (typeof previousCell === "object" && currentCell.equalColour(previousCell)) {
                        currentSequence.count++;
                    } else { // Sequence broken
                        maxSequence.updateIfGreater(currentSequence);
                        currentSequence.reset(currentX, currentY);
                        currentSequence.count = 1;
                    }
                }
            } else { // Cell is empty
                maxSequence.updateIfGreater(currentSequence);
                currentSequence.reset(currentX, currentY); // Reset, count remains 0
            }
        }
        maxSequence.updateIfGreater(currentSequence);
        return maxSequence;
    }

    /**
     * Clears a diagonal line (top-left to bottom-right) from the grid.
     * @param {ContinuousColorBalls} lineInfo - Information about the diagonal line to clear.
     */
    clearDiagonalRightLine(lineInfo) {
        for (let i = 0; i < lineInfo.count; i++) {
            this.grid.data[lineInfo.y + i][lineInfo.x + i] = 0;
        }
    }
    /**
     * Checks all possible top-left to bottom-right diagonals for lines and clears them.
     * Diagonals are checked starting from the top edge (y=0, x varying) and then from the left edge (x=0, y varying).
     * A line must have at least 5 balls.
     * @returns {boolean} True if any diagonal line was found and cleared, false otherwise.
     */
    checkAndClearDiagonalRightLines() {
        let lineFound = false;
        // Iterate over all possible starting points for right diagonals.
        // A diagonal needs at least 5 cells to form a line.
        // Start checking diagonals from the top row (y=0), moving x from 0 up to grid.size - 5
        for (let startX = 0; startX <= this.grid.size - 5; startX++) {
            const longestLine = this.findLongestDiagonalRightSequence(startX, 0);
            if (longestLine.isFiveOrMore()) {
                this.clearDiagonalRightLine(longestLine);
                Utils.increaseScore(longestLine.count);
                this.score += longestLine.count;
                lineFound = true;
            }
        }
        // Start checking diagonals from the left column (x=0), moving y from 1 (y=0 already covered) up to grid.size - 5
        for (let startY = 1; startY <= this.grid.size - 5; startY++) {
            const longestLine = this.findLongestDiagonalRightSequence(0, startY);
            if (longestLine.isFiveOrMore()) {
                this.clearDiagonalRightLine(longestLine);
                Utils.increaseScore(longestLine.count);
                this.score += longestLine.count;
                lineFound = true;
            }
        }
        return lineFound;
    }


    /**
     * Finds the longest sequence of same-colored balls along a top-right to bottom-left diagonal.
     * @param {number} startX - The starting x-coordinate of the diagonal.
     * @param {number} startY - The starting y-coordinate of the diagonal.
     * @returns {ContinuousColorBalls} The longest sequence found on this diagonal.
     */
    findLongestDiagonalLeftSequence(startX, startY) {
        let currentSequence = new ContinuousColorBalls(startX, startY, 0);
        let maxSequence = new ContinuousColorBalls(startX, startY, 0);
        // Determine how many steps can be taken: limited by startX (towards 0) and grid height (towards grid.size -1)
        const maxSteps = Math.min(startX + 1, this.grid.size - startY);

        for (let step = 0; step < maxSteps; step++) {
            const currentX = startX - step;
            const currentY = startY + step;
            const currentCell = this.grid.data[currentY][currentX];

            if (typeof currentCell === "object") { // Cell contains a ball
                if (currentSequence.count === 0) { // New sequence
                    currentSequence.reset(currentX, currentY);
                    currentSequence.count = 1;
                } else {
                    // Previous cell in a top-right to bottom-left diagonal is (y-1, x+1)
                    const previousCell = this.grid.data[currentY - 1][currentX + 1];
                    if (typeof previousCell === "object" && currentCell.equalColour(previousCell)) {
                        currentSequence.count++;
                    } else { // Sequence broken
                        maxSequence.updateIfGreater(currentSequence);
                        currentSequence.reset(currentX, currentY);
                        currentSequence.count = 1;
                    }
                }
            } else { // Cell is empty
                maxSequence.updateIfGreater(currentSequence);
                currentSequence.reset(currentX, currentY);
            }
        }
        maxSequence.updateIfGreater(currentSequence);
        return maxSequence;
    }

    /**
     * Clears a diagonal line (top-right to bottom-left) from the grid.
     * @param {ContinuousColorBalls} lineInfo - Information about the diagonal line to clear.
     */
    clearDiagonalLeftLine(lineInfo) {
        for (let i = 0; i < lineInfo.count; i++) {
            this.grid.data[lineInfo.y + i][lineInfo.x - i] = 0;
        }
    }

    /**
     * Checks all possible top-right to bottom-left diagonals for lines and clears them.
     * A line must have at least 5 balls.
     * Diagonals are checked starting from the top edge (y=0, x varying from grid.size-1 down to 4),
     * and then from the right edge (x=grid.size-1, y varying from 1 up to grid.size-5).
     * The starting x for y=0 is `grid.size - 1` down to `4` because a diagonal starting at x < 4 with y=0
     * cannot have a length of 5. Similar logic applies to starting y for x=grid.size-1.
     * @returns {boolean} True if any diagonal line was found and cleared, false otherwise.
     */
    checkAndClearDiagonalLeftLines() {
        let lineFound = false;
        // Check diagonals starting from the top row (y=0).
        // startX goes from grid.size-1 down to 4 (inclusive, as a line needs 5 balls).
        // A diagonal from (startX, 0) goes to (startX-N, N). Smallest startX is 4 for a 5-ball line: (4,0) to (0,4).
        for (let startX = this.grid.size - 1; startX >= 4; startX--) {
            const longestLine = this.findLongestDiagonalLeftSequence(startX, 0);
            if (longestLine.isFiveOrMore()) {
                this.clearDiagonalLeftLine(longestLine);
                Utils.increaseScore(longestLine.count);
                this.score += longestLine.count;
                lineFound = true;
            }
        }

        // Check diagonals starting from the rightmost column (x=this.grid.size-1).
        // startY goes from 1 (y=0 already covered) up to this.grid.size - 5.
        // Smallest startY is 1. Largest startY is grid.size-5 for a 5-ball line: (size-1, size-5) to (size-5, size-1).
        for (let startY = 1; startY <= this.grid.size - 5; startY++) {
            const longestLine = this.findLongestDiagonalLeftSequence(this.grid.size - 1, startY);
            if (longestLine.isFiveOrMore()) {
                this.clearDiagonalLeftLine(longestLine);
                Utils.increaseScore(longestLine.count);
                this.score += longestLine.count;
                lineFound = true;
            }
        }
        return lineFound;
    }

    /**
     * Scans the grid for all types of lines (horizontal, vertical, diagonals)
     * and clears them.
     * @returns {boolean} True if any line was found and cleared, false otherwise.
     */
    scanLines() {
        const horizontalFound = this.checkAndClearHorizontalLines();
        const verticalFound = this.checkAndClearVerticalLines();
        const diagRightFound = this.checkAndClearDiagonalRightLines();
        const diagLeftFound = this.checkAndClearDiagonalLeftLines();
        return horizontalFound || verticalFound || diagRightFound || diagLeftFound;
    }
}