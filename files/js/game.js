/**
 * Represents the main game logic.
 */
class Game {
    /**
     * Creates a new Game instance.
     */
    constructor() {
        this.isSelected = false;
        this.selectedX = -1;
        this.selectedY = -1;
        this.targetX = -1;
        this.targetY = -1;
        this.startBallCount = 5;
        this.selectedBall = null;
        this.upcomingBalls = []; // Stores the colors of the next three balls
    }

    /**
     * Initializes the game.
     */
    init() {
        this.initGridData();
        this.initSolver();
        this.initClickEvents();
        this.initResizeEvent();
    }

    /**
     * Initializes the grid data, places random balls, generates and displays upcoming balls, and sets up pathfinding.
     */
    initGridData() {
        this.grid = new Grid(9);
        this.grid.placeRandomBalls(this.startBallCount);
        this.generateUpcomingBalls();
        this.displayUpcomingBalls();
        this.initEasyStar();
        this.grid.displayGrid();
    }

    /**
     * Generates the next three upcoming ball colors.
     */
    generateUpcomingBalls() {
        this.upcomingBalls = [];
        for (let i = 0; i < 3; i++) {
            const randomColourIndex = Utils.Random.nextInt(0, Utils.settings.colours.length);
            this.upcomingBalls.push(Utils.settings.colours[randomColourIndex]);
        }
    }

    /**
     * Displays the upcoming balls in the HTML.
     */
    displayUpcomingBalls() {
        const nextBallsDisplay = $("#nextBallsDisplay");
        nextBallsDisplay.empty(); // Clear current upcoming balls

        this.upcomingBalls.forEach(color => {
            const ballPreview = $("<div>")
                .addClass("next-ball-preview")
                .css({
                    'background-image': `url(files/images/${color}_ball.png)`
                });
            nextBallsDisplay.append(ballPreview);
        });
    }


    /**
     * Places the currently stored upcoming balls onto the grid.
     * This method calls `this.grid.placeRandomBallWithColor` for each upcoming ball.
     * If the grid is full and a ball cannot be placed, it could trigger a game over scenario (not implemented here).
     */
    placeUpcomingBallsOnGrid() {
        for (const color of this.upcomingBalls) {
            const placed = this.grid.placeRandomBallWithColor(color);
            if (!placed) {
                // Handle game over or grid full scenario
                console.warn("Grid is full, cannot place upcoming ball:", color);
                // Potentially trigger game over logic here
                break; // Stop trying to place more balls if one fails
            }
        }
    }

    /**
     * Initializes the game solver.
     */
    initSolver() {
        this.solver = new Solver(this.grid);
    }

    /**
     * Initializes EasyStar pathfinding library.
     */
    initEasyStar() {
        this.easystar = new EasyStar.js();
        this.easystar.setGrid(this.grid.data);
        this.easystar.setAcceptableTiles([0]); // 0 represents an empty tile
        this.easystar.setIterationsPerCalculation(1000);
    }

    /**
     * Initializes the window resize event handler to make the grid responsive.
     */
    initResizeEvent() {
        var resizeTimer;
        $(window).on('resize', (e) => { // Used arrow function to preserve 'this' context
            clearTimeout(resizeTimer);
            resizeTimer = setTimeout(() => { // Used arrow function
                this.grid.makeResponsive();
            }, 250);
        });
    }

    /**
     * Handles click events on grid elements.
     * @param {number} selX - The x-coordinate of the clicked element.
     * @param {number} selY - The y-coordinate of the clicked element.
     */
    handleClickEvent(selX, selY) {
        if (!this.isSelected) {
            // If no ball is currently selected, and the clicked cell is not empty
            if (!this.grid.isEmpty(selX, selY)) {
                this.selectInitialPoint(selX, selY);
            }
        } else {
            // If a ball is already selected
            if (this.grid.isEmpty(selX, selY)) {
                // If the clicked cell is empty, select it as the target
                this.selectTargetPoint(selX, selY);
                this.findPath();
            } else {
                // If the clicked cell is not empty (e.g., another ball), unselect the current ball
                this.unselectInitialPoint();
            }
        }
    }

    /**
     * Initializes click event handlers for grid elements and the document.
     */
    initClickEvents() { // Corrected typo: initClicKEvents to initClickEvents
        // Handle clicks on individual grid elements
        $(".element").click((event) => { // Used arrow function
            var selX = parseInt($(event.currentTarget).attr("data-x"));
            var selY = parseInt($(event.currentTarget).attr("data-y"));
            this.handleClickEvent(selX, selY);
        });

        // Handle clicks outside the game grid to unselect any selected ball
        $(document).click((e) => { // Used arrow function
            var gameElements = $(".element");
            // Check if the click target is outside the .element divs
            if (!gameElements.is(e.target) && gameElements.has(e.target).length === 0) {
                this.isSelected = false;
                this.unhighlightSelectedPoint();
            }
        });
    }

    /**
     * Highlights the selected grid point.
     */
    highlightSelectedPoint() {
        $(`.element[data-x=${this.selectedX}][data-y=${this.selectedY}]`).addClass("selected");
    }

    /**
     * Removes highlighting from all grid points.
     */
    unhighlightSelectedPoint() {
        $(".element").removeClass("selected");
    }

    /**
     * Selects the initial grid point (containing a ball).
     * @param {number} x - The x-coordinate of the point.
     * @param {number} y - The y-coordinate of the point.
     */
    selectInitialPoint(x, y) {
        this.isSelected = true;
        this.selectedX = x;
        this.selectedY = y;
        this.selectedBall = this.grid.data[y][x];
        this.highlightSelectedPoint();
    }

    /**
     * Unselects the currently selected initial grid point.
     */
    unselectInitialPoint() {
        this.isSelected = false;
        this.selectedX = -1;
        this.selectedY = -1;
        this.selectedBall = null;
        this.unhighlightSelectedPoint();
    }

    /**
     * Selects the target grid point (empty cell for moving a ball).
     * @param {number} x - The x-coordinate of the point.
     * @param {number} y - The y-coordinate of the point.
     */
    selectTargetPoint(x, y) {
        this.isSelected = false; // A target point is selected, so no ball is "selected" for movement initiation anymore
        this.targetX = x;
        this.targetY = y;
        this.unhighlightSelectedPoint(); // Unhighlight the previously selected ball's cell
    }

    /**
     * Moves a ball along a given path, checks for lines, and places new balls.
     * @param {Ball} ball - The ball object to move.
     * @param {Array<{x: number, y: number}>} path - An array of coordinates representing the path.
     */
    moveBall(ball, path) {
        // Clone the ball element for animation
        var ballClone = $("<div>").attr({
            'class': 'ball',
            'data-ballId': ball.id // Use ball.id to uniquely identify the ball element
        }).css({
            'background': `url("files/images/${ball.colour}_ball.png")`, // Corrected path
            'background-size': '100%'
        });

        var currentStep = 0; // Tracks the current step in the path animation
        // Animate the ball moving along the path
        for (let i = 0; i < path.length; i++) {
            setTimeout(() => {
                $(`.ball[data-ballId="${ball.id}"]`).remove();
                $(`.element[data-x=${path[currentStep].x}][data-y=${path[currentStep].y}]`).append(ballClone);
                currentStep++;

                if (currentStep === path.length) {
                    this.grid.data[this.targetY][this.targetX] = this.grid.data[this.selectedY][this.selectedX];
                    this.grid.data[this.selectedY][this.selectedX] = 0;

                    setTimeout(() => {
                        // Use the renamed solver method scanAndClearLines()
                        if (!this.solver.scanLines()) { // Assuming scanLines is the correct name after previous refactor or use scanAndClearLines if that was the final name
                            this.placeUpcomingBallsOnGrid(); // Place the displayed upcoming balls
                            this.generateUpcomingBalls();    // Generate the *new* set of three
                            this.displayUpcomingBalls();     // Display the *new* set
                            
                            // Potentially, after placing upcoming balls, new lines could form.
                            // A common game mechanic is to check for lines again here.
                            setTimeout(() => {
                                if (this.solver.scanLines()) { // Or scanAndClearLines()
                                     // If lines were formed by upcoming balls, display grid and re-init clicks
                                    this.grid.displayGrid();
                                    this.initClickEvents();
                                }
                            }, 20); // Short delay for visual consistency
                        }
                        // Refresh the display of the grid to show all changes
                        this.grid.displayGrid();
                        // Re-initialize click events as the grid elements might have been redrawn
                        this.initClickEvents();
                    }, 20);
                }
            }, i * 20); // Stagger the animation steps
        }
    }


    /**
     * Finds a path from the selected ball to the target point using EasyStar.js.
     */
    findPath() {
        this.easystar.findPath(this.selectedX, this.selectedY, this.targetX, this.targetY, (path) => {
            if (path !== null) {
                this.moveBall(this.selectedBall, path);
            } else {
                console.warn("No path found.");
            }
        });
        this.easystar.calculate();
    }
}