/**
 * Utility functions for the game.
 * @namespace Utils
 */
var Utils = {
    /**
     * Random number generation functions.
     * @namespace Utils.Random
     */
    Random: {
        /**
         * Generates a random integer within a specified range.
         * @param {number} min - The minimum value (inclusive).
         * @param {number} max - The maximum value (exclusive).
         * @returns {number} A random integer.
         */
        nextInt(min, max) {
            return Math.floor(Math.random() * max) + min;
        }
    },
    /**
     * Game settings.
     * @namespace Utils.settings
     */
    settings: {
        /**
         * Available ball colours.
         * @type {string[]}
         */
        colours: ["red", "blue", "yellow", "green", "brown"]
        //colours: ["red", "blue", "yellow", "green", "purple", "turquoise", "pink", "brown"]
    },
    /**
     * Creates an empty grid of a given size.
     * @param {number} size - The size of the grid (width and height).
     * @returns {Array<Array<number>>} A 2D array representing the grid, initialized with zeros.
     */
    makeGrid: size => {
        var grid = [];
        for (var i = 0; i < size; i++) {
            var row = [];
            for (var y = 0; y < size; y++) {
                row.push(0);
            }
            grid.push(row);
        }
        return grid;
    },
    /**
     * Increases the player's score.
     * @param {number} num - The amount to increase the score by.
     */
    increaseScore: (num) => {
        var score = parseInt($('#score').text());
        score += num;
        $('#score').text(score);
    }
};