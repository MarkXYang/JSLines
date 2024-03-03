var Utils = {
    Random: {
        nextInt(min, max) {
            return Math.floor(Math.random() * max) + min;
        }
    },
    settings: {
        colours: ["red", "blue", "yellow", "green", "brown"]
        //colours: ["red", "blue", "yellow", "green", "purple", "turquoise", "pink", "brown"]
    },
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
    showPath: path => {
        for (var x = 0; x < path.length; x++) {
            $(`.element[data-x=${path[x].x}][data-y=${path[x].y}]`).addClass("path");
        }
    },
    moveBall: (ball, path) => {
        var ball = $("");
        for (var i = 0; i < path.length; i++) {
            $(`.element[data-x=${path[x].x}][data-y=${path[x].y}]`);
        }
    },
    clearPath: () => {
        $(".element").removeClass("path");
    },
    increaseScore: (score) => {
        for(var i = 0; i < 100; i++) {
            setTimeout(() => {
                score++;
                $('#score').text(score);
            }, i * 5)
        }
    }
};