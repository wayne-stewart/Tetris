
var GameBoard = new Object();

(function(gb) {

	var Square = function() {
		var _color = 0;
		this.get_color = function() {
			return _color;
		};
		this.set_color = function(c) {
			_color = c;
		};
	};
	
	var Shape = function(x,y,c,squares) {
		var _squares = squares;
		var _x = x;
		var _y = y;
		var _c = c;
		
		this.get_X = function() { return _x; };
		this.get_Y = function() { return _y; };
		this.get_color = function() { return _c; };
		this.get_squares = function() { return _squares; };
		
		this.shiftUp = function() { _y--; };
		this.shiftDown = function() { _y++; };
		this.shiftRight = function() { _x++; };
		this.shiftLeft = function() { _x--; };
		this.rotate90 = function() {
			for (var i in _squares) {
				var x = _squares[i].x * 0 - _squares[i].y * 1; // x cos(90) - y sin(90)
				var y = _squares[i].x * 1 + _squares[i].y * 0; // x sin(90) + y cos(90)
				_squares[i].x = x;
				_squares[i].y = y;
			}
		};
		this.rotate270 = function() {
			for (var i in _squares) {
				var x = _squares[i].x * 0 - _squares[i].y * -1; // x cos(270) - y sin(270)
				var y = _squares[i].x * -1 + _squares[i].y * 0; // x sin(270) + y cos(270)
				_squares[i].x = x;
				_squares[i].y = y;
			}
		};
	};
	
	var ShapeFactory = function() {
		var _x;
		this.set_newShapeX = function(x) { _x = x; };
		
		this.create = function() {
			var r = Math.floor(Math.random() * 7);
			var c = 7;//Math.ceil(Math.random() * 6);
			var squares;
			switch (r) {
				case 0: // vertical line
					squares = [{x:0,y:-2},{x:0,y:-1},{x:0,y:0},{x:0,y:1}];
					break;
				case 1: // right L
					squares = [{x:0,y:-1},{x:0,y:0},{x:0,y:1},{x:1,y:1}];
					break;
				case 2: // left L
					squares = [{x:0,y:-1},{x:0,y:0},{x:0,y:1},{x:-1,y:1}];
					break;
				case 3: // square
					squares = [{x:0,y:0},{x:0,y:1},{x:1,y:0},{x:1,y:1}];
					break;
				case 4: // t
					squares = [{x:0,y:-1},{x:0,y:0},{x:0,y:1},{x:1,y:0}];
					break;
				case 5: // right rhombus
					squares = [{x:-1,y:-1},{x:0,y:-1},{x:0,y:0},{x:1,y:0}];
					break;
				case 6: // left rhombus
					squares = [{x:1,y:-1},{x:0,y:-1},{x:0,y:0},{x:-1,y:0}];
					break;
			}
				
			var s = new Shape(_x, 0, c, squares);
			
			// squares don't rotate
			if (r == 3) {
				s.rotate90 = function(d) { };
				s.rotate270 = function(d) { };
			}
			return s;
		};
	};

	var Board = function(width, height) {
		
		var _self = this;
		var _width = width;
		var _height = height;
		var _grid = new Array();
		var _nextShape;
		var _currentShape;
		var _state = 0;
		var _level = 1;
		var _rowsToNextLevel = 5;
		var _activeSquares = 0;
		var _completedRows = 0;
		var _score = 0;
		var _shapeFactory = new ShapeFactory();
		_shapeFactory.set_newShapeX(Math.floor(_width / 2));
		
		for (var j = 0; j < height; ++j) {
			_grid.push(new Array());
			for (var i = 0; i < width; ++i) {
				_grid[j][i] = new Square();
				_grid[j][i].set_color(0);
			}
		}
		
		this.get_width = function() { return _width; };
		this.get_height = function() { return _height; };
		this.get_grid = function() { return _grid; };
		this.get_currentShape = function() { return _currentShape; };
		this.get_nextShape = function() { return _nextShape; };
		this.get_IsStarted = function() { return _state > 0; };
		this.get_IsPlaying = function() { return _state == 1; };
		this.get_IsPaused = function() { return _state == 2; };
		this.get_level = function() { return _level; };
		this.get_score = function() { return _score; };
		this.start = function() { _state = 1; };
		this.pause = function() { _state = 2; };
		
		this.levelIncrease = function() { };
		this.bonusScore = function(bonus) { };
		this.callback_score = function(bonus) { };
		
		this.setup = function() {
			_level = 1;
			_score = 0;
			_rowsToNextLevel = 5;
			_completedRows = 0;
			_activeSquares = 0;
			_currentShape = _shapeFactory.create();
			_nextShape = _shapeFactory.create();
			
			// clear game board
			for (var j = 0; j < height; ++j) {
				for (var i = 0; i < width; ++i) {
					_grid[j][i].set_color(0);
				}
			}
		};
		
		this.tryRotate = function() {
			_currentShape.rotate90();
			if (validateMove()) {
				return true;
			} else {
				_currentShape.rotate270(); // reverse the action
				return false;
			}
		};
		
		this.tryShiftDown = function() {
			_currentShape.shiftDown();
			if (validateMove()) {
				return true;
			}
			else {
				_currentShape.shiftUp();  // reverse the action
				setShapeInGrid(_currentShape);
				if (_state == 1) {
					checkForCompletedLines(_currentShape);
					_currentShape = _nextShape;
					_nextShape = _shapeFactory.create();
				}
				return false;
			}
		};
		
		this.tryShiftRight = function() {
			_currentShape.shiftRight();
			if (validateMove()) {
				return true;
			} else {
				_currentShape.shiftLeft();  // reverse the action
				return false;
			}
		};
		
		this.tryShiftLeft = function() {
			_currentShape.shiftLeft();
			if (validateMove()) {
				return true;
			} else {
				_currentShape.shiftRight();  // reverse the action
				return false;
			}
		};
		
		var validateMove = function() {
			var x = _currentShape.get_X();
			var y = _currentShape.get_Y();
			var squares = _currentShape.get_squares();
			for (var index in squares) {
				var xx = x + squares[index].x;
				var yy = y + squares[index].y;
				
				// only validate visible squares
				if (yy < 0)
					return true;
					
				if (yy >= _height)
					return false;
				
				if (xx < 0)
					return false;
					
				if (xx >= _width)
					return false;
					
				if (_grid[yy][xx].get_color() > 0)
					return false;
			}
			return true;
		};
		
		var setShapeInGrid = function(shape) {
			var x = shape.get_X();
			var y = shape.get_Y();
			var squares = shape.get_squares();
			_activeSquares += squares.length;
			for (var index in squares) {
				var xx = x + squares[index].x;
				var yy = y + squares[index].y;
				if (yy < 0) {
					_state = 2;
				} else {
					_grid[yy][xx].set_color(shape.get_color());
				}
			}
		};
		
		var checkForCompletedLines = function(shape) {
			var y = shape.get_Y();
			var squares = shape.get_squares();
			var completed = new Array();
			for (var i in squares) {
				var yy = y + squares[i].y;
				if (completed.indexOf(yy) == -1) {
					var isComplete = true;
					for (var j = 0; j < _width; ++j) {
						if (_grid[yy][j].get_color() == 0) {
							isComplete = false;
						}
					}
					if (isComplete) {
						completed.push(yy);
					}
				}
			}

			if (completed.length > 0) {
				completed.sort();
				var removed = new Array();
				for (var i = completed.length - 1; i >= 0; --i) {
					var b = _grid.splice(completed[i], 1)[0];
					removed.push(b);
				}
				for (var i in removed) {
					for (var j in removed[i])
						removed[i][j].set_color(0);
					_grid.splice(0,0,removed[i]);
				}
				
				updateScore(completed.length);
			}
		};
		
		var updateScore = function(rowCount) {
			_completedRows += rowCount;
			_activeSquares -= rowCount * _width;
			_score += rowCount * rowCount * _level;
			_rowsToNextLevel -= rowCount;
			
			// bonus if you clear the screen
			if (_activeSquares == 0) {
				var bonus = 10 * rowCount * _level;
				_score += bonus;
				_self.bonusScore(bonus);
			}
			
			// progress to next level
			if (_rowsToNextLevel <= 0) {
				_level++;
				_rowsToNextLevel = 5 + _level * 5;
				_self.levelIncrease();
			}
		};
	};

	gb.create = function(width, height) {
		return new Board(width, height);
	};

})(GameBoard);