/**
 * Represents the game grid.
 */
class Grid {
    /**
     * Creates a new Grid instance.
     * @param {number} size - The size of the grid (width and height).
     */
    constructor(size) {
        this.size = size;
        /**
         * 2D array representing the grid data. 0 for empty, Ball object otherwise.
         * @type {Array<Array<Ball|number>>}
         */
        this.data = Utils.makeGrid(size);
        /**
         * Counter for the total number of balls placed on the grid (used for unique ball IDs).
         * @type {number}
         */
        this.ballCount = 0;
        // this.emptyCells = []; // This property was initialized but not used. Consider removing if not needed.
    }

    /**
     * Builds the HTML for a single cell, including a ball if present.
     * @param {number} rowIndex - The row index of the cell.
     * @param {number} colIndex - The column index of the cell.
     * @returns {jQuery|null} A jQuery object representing the ball element, or null if the cell is empty.
     */
    buildCell(rowIndex, colIndex) {
        const cellData = this.data[rowIndex][colIndex];
        if (cellData === 0) { // Cell is empty
            return null;
        } else { // Cell contains a ball
            return $("<div>").addClass(`ball`).css({
                'background': `url("files/images/${cellData.colour}_ball.png")`,
                'background-size': '100%'
            }).attr({
                'data-ballId': `${cellData.id}`, // Unique ID for the ball element
                'title': cellData.colour // Tooltip showing the ball's colour
            });
        }
    }

    /**
     * Renders the entire grid in the HTML.
     * It clears the existing grid and rebuilds it based on the current `this.data`.
     */
    displayGrid() {
        // Remove existing grid elements to prevent duplication
        $('.element').remove();

        for (var rowIndex = 0; rowIndex < this.data.length; rowIndex++) {
            for (var colIndex = 0; colIndex < this.data[rowIndex].length; colIndex++) {
                // Create a div for each cell in the grid
                var element = $("<div>")
                    .addClass("element") // Base class for styling grid cells
                    .attr({
                        "data-x": colIndex, // Store column index, used for click handling and logic
                        "data-y": rowIndex  // Store row index, used for click handling and logic
                    })
                    .append(this.buildCell(rowIndex, colIndex)); // Add ball to cell if present

                // Add 'first' class to the first cell in each row, potentially for styling (e.g., removing left border)
                if (colIndex === 0) {
                    element.addClass("first");
                }
                $("#game").append(element); // Append the cell to the main game container
            }
        }
        this.makeResponsive(); // Adjust cell sizes after display
    }

    /**
     * Adjusts the size of the grid cells to fit the viewport width,
     * primarily for smaller screens where the body width might be less than the window height.
     */
    makeResponsive() {
        var bodyWidth = $('body').width();
        // Only apply responsive sizing if the body width is less than the window height (common on portrait mobile)
        if (bodyWidth < $(window).height()) {
            var padding = 3 * 2; // Assumes 3px padding on each side (left/right or top/bottom)
            var borderSize = 1 * 2; // Assumes 1px border on each side
            // Calculate the available width per cell
            var cellSize = (bodyWidth - (this.size * padding) - (this.size * borderSize)) / this.size;

            // console.log(`bw: ${bodyWidth}, padding: ${padding}, borderSize: ${borderSize}, cellSize: ${cellSize}`);

            // Inject a style rule to set the width and height of .element divs.
            // This method is used instead of directly applying .css() to potentially avoid visual glitches
            // during ball movement animations, as direct .css() changes might reflow the page more disruptively.
            $('#responsive').text(`.element { width: ${cellSize}px; height:${cellSize}px}`);
        }
    }

    /**
     * Checks if a cell at the given coordinates is empty.
     * @param {number} x - The x-coordinate (column index).
     * @param {number} y - The y-coordinate (row index).
     * @returns {boolean} True if the cell is empty (contains 0), false otherwise.
     */
    isEmpty(x, y) {
        return this.data[y][x] === 0;
    }

    /**
     * Logs the current grid state to the console in a formatted way. (Primarily for debugging)
     */
    fancyLog() {
        for (var i = 0; i < this.data.length; i++) { // Iterate over rows
            var line = `${i}: `;
            for (var y = 0; y < this.data[i].length; y++) { // Iterate over columns in the current row
                // Note: Original code had `this.data[y][i]` which would transpose the grid if y iterates up to data[i].length
                // Assuming it's meant to be this.data[i][y] for standard row-column logging.
                // If it was intentionally transposed, a comment should clarify.
                // For now, corrected to this.data[i][y] for clarity.
                line += `| ${this.data[i][y] instanceof Ball ? this.data[i][y].toString() : this.data[i][y]}`;
            }
            console.log(line);
        }
    }

    /**
     * Places a specified number of random balls on the grid.
     * @param {number} count - The number of random balls to place.
     */
    placeRandomBalls(count) {
        for (var i = 0; i < count; i++) {
            this.placeRandomBall();
        }
    }

    /**
     * Places a single random ball on an empty cell in the grid.
     * If the randomly chosen cell is already occupied, it recursively tries again.
     */
    placeRandomBall() {
        // Generate random coordinates within the grid boundaries.
        // Note: size-1 might be an issue if Utils.Random.nextInt's max is exclusive.
        // If nextInt(min, max) is [min, max), then size is correct. If [min, max], then size-1 is needed.
        // Assuming Utils.Random.nextInt(0, this.size) would be [0, this.size-1]
        var randomRowIndex = Utils.Random.nextInt(0, this.size);
        var randomColIndex = Utils.Random.nextInt(0, this.size);

        if (this.data[randomRowIndex][randomColIndex] === 0) { // Check if the cell is empty
            this.ballCount++; // Increment total ball count for unique ID generation
            const randomColour = Utils.settings.colours[Utils.Random.nextInt(0, Utils.settings.colours.length)];
            this.data[randomRowIndex][randomColIndex] = new Ball(randomColour, this.ballCount);
        } else {
            // If the cell is occupied, try placing the ball again.
            // This recursive call could lead to a stack overflow if the grid is nearly full.
            // A safer approach for very full grids would be to find all empty cells and pick one.
            this.placeRandomBall(); // Consider replacing recursion with a loop or finding all empty cells
        }
    }

    /**
     * Places a single ball of a specific color on a random empty cell in the grid.
     * If the randomly chosen cell is already occupied, it recursively tries again.
     * @param {string} colorName - The color of the ball to place.
     * @returns {boolean} True if the ball was placed, false if the grid is full.
     */
    placeRandomBallWithColor(colorName) {
        let attempts = 0;
        const maxAttempts = this.size * this.size; // Limit attempts to prevent infinite loop if grid is full

        while (attempts < maxAttempts) {
            var randomRowIndex = Utils.Random.nextInt(0, this.size);
            var randomColIndex = Utils.Random.nextInt(0, this.size);

            if (this.data[randomRowIndex][randomColIndex] === 0) { // Check if the cell is empty
                this.ballCount++; // Increment total ball count for unique ID generation
                this.data[randomRowIndex][randomColIndex] = new Ball(colorName, this.ballCount);
                return true; // Ball placed successfully
            }
            attempts++;
        }
        console.warn("Could not find an empty cell to place ball with color:", colorName);
        return false; // Could not place the ball (grid might be full)
    }
}