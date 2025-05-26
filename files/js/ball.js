/**
 * Represents a ball in the game.
 */
class Ball {
  /**
   * Creates a new Ball instance.
   * @param {string} colour - The colour of the ball.
   * @param {number} id - The unique identifier of the ball.
   */
  constructor(colour, id) {
    this.id = id;
    this.colour = colour;
  }

  /**
   * Returns a string representation of the ball.
   * @returns {string} A string in the format "id: colour".
   */
  toString() {
    return `${this.id}: ${this.colour}`;
  }

  /**
   * Checks if this ball has the same colour as another ball.
   * @param {Ball} ball - The other ball to compare with.
   * @returns {boolean} True if the balls have the same colour, false otherwise.
   */
  equalColour(ball) {
    return this.colour === ball.colour;
  }
}
