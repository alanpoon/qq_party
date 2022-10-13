/* jQuery sorted voting system */
/* Provides automatic sorting and voting on elements
 * © Pez Cuckow 2012+
 * email@pezcuckow.com
 * Please keep attribution, including use of modified or selected code.
 * Twitter: @Pezmc
 */
$(function() {
    var updating = false;

    function voteClick(button, up, table) {
        if (!updating) {
            updating = true;
            $("html").trigger('startUpdate');
            var cell = $('td:nth-child(2)', $(button).parent().parent());
            if (up) cell.text((parseInt(cell.text()) + 1)); //add ajax
            else cell.text((parseInt(cell.text()) - 1)); //add ajax etc here
            sortTable(table, function() {
                updating = false;
                $("html").trigger('stopUpdate');
            }); //callback
        }
    }
    window.leaderboard_new_data=function(newData){
        if (!updating) {
            updating = true;
            $("html").trigger('startUpdate');
            var table = $("#table tbody");
            for (var i=0;i<newData.length;i++){
                var found = false;
                table.children().each(function(n,m){
                    var cell = $('td:nth-child(3)', $(m)).text();
                    if (cell == newData[i].name){
                        found = true
                        $('td:nth-child(2)',$(m)).text(newData[i].score)
                    }
                });
                if (!found){
                    var s = "<tr>" + 
                    "<td>"+i+"</td>"+
                    "<td>"+newData[i].score+"</td>"+
                    "<td class='name f32'>" + '<img class="flag '+newData[i].flag+'"/>'  +newData[i].name + "</td>" + 
                    "</tr>";
                    table.append(s);
                }
            }
            sortTable($("#table"), function() {
                updating = false;
                $("html").trigger('stopUpdate');
            }); 
        }
        
    }
    // var updateInterval = 3000;
    // timerId = setInterval(getNewData, updateInterval);

    function makeClickable(table) {
        $('.up', table).each(function() {
            $(this).css('cursor', 'pointer').click(function() {
                voteClick(this, true, table);
            });
        });
        $('.down', table).each(function() {
            $(this).css('cursor', 'pointer').click(function() {
                voteClick(this, false, table);
            });
        });
        $('thead tr th').each(function() {
            $(this).css('cursor', 'pointer').click(function() {
                updating = true;
                $("html").trigger('startUpdate');

                //Current sort
                $(".anim\\:sort", $(this).parent()).removeClass("anim:sort");
                $(this).addClass("anim:sort");

                sortTable(table, function() {
                    updating = false;
                    $("html").trigger('stopUpdate');
                }); //callback
            })
        });
    }

    function isNumber(n) {
        return !isNaN(parseFloat(n)) && isFinite(n);
    }

    var inverse = false;

    function compareCells(a, b) {
        var b = $.text([b]);
        var a = $.text([a]);

        if (isNumber(a) && isNumber(b)) {
            return parseInt(b) - parseInt(a);
        } else {
            return a.localeCompare(b);
        }
    }

    /**
     * Update the ranks (1-n) of a table
     * @param table A jQuery table object
     * @param index The row index to put the positions in
     */

    function updateRank(table, index) {
        var position = 1;
        if (!index) index = 1;

        $("tbody tr", table).each(function() {
            var cell = $("td:nth-child(" + index + ")", this);
            if (parseInt(cell.text()) != position) cell.text(position); //only change if needed
            position++;
        });
    }

    /**
     * jQuery compare arrays
     */
    jQuery.fn.compare = function(t) {
        if (this.length != t.length) {
            return false;
        }
        var a = this,
            b = t;
        for (var i = 0; t[i]; i++) {
            if (a[i] !== b[i]) {
                return false;
            }
        }
        return true;
    };

    /**
     * Sort a provided table by a row
     * @param currentTable A jQuery table object
     * @param index The row index to sort on
     */

    function sortTable(currentTable, callback) {
        var newTable = currentTable.clone();
        newTable.hide();
        $('.demo').append(newTable);

        //What one are we ordering on?
        var sortIndex = $(newTable).find(".anim\\:sort").index();

        //Old table order
        var idIndex = $(newTable).find(".anim\\:id").index();
        var startList = newTable.find('td').filter(function() {
            return $(this).index() === idIndex;
        });

        //Sort the list
        newTable.find('td').filter(function() {
            return $(this).index() === sortIndex;
        }).sortElements(compareCells, function() { // parentNode is the element we want to move
            return this.parentNode;
        });

        //New table order
        var idIndex = $(newTable).find(".anim\\:id").index();
        var endList = newTable.find('td').filter(function() {
            return $(this).index() === idIndex;
        });

        if (!$(startList).compare(endList)) { //has the order actually changed?        
            makeClickable(newTable);
            updateRank(newTable);
            if (!callback) currentTable.rankingTableUpdate(newTable);
            else {
                currentTable.rankingTableUpdate(newTable, {
                    onComplete: callback
                });
            }
        } else {
            callback(); //we're done
        }
        console.log("newtable",$(newTable));
        $(newTable).find("tr").slice(8).remove();
        $(currentTable).find("tr").slice(8).remove();
    }
  
    // Do the work!
    makeClickable($('#table'));
});

$(function() {
    $("html").bind("startUpdate", function() {
        $(this).addClass('busy');
    }).bind("stopUpdate", function() {
        $(this).removeClass('busy');
    });
});


///////////////////SORTTABLES.JS//////////////////////////////////////
/* Add stable merge sort to Array and jQuery prototypes */
(function() {
    // expose to Array and jQuery
    Array.prototype.msort = jQuery.fn.msort = msort;

    // the actual compare


    function msort(compare) {
        var length = this.length,
            middle = Math.floor(length / 2);
        if (!compare) {
            compare = function(left, right) {
                if (left < right) return -1;
                if (left == right) return 0;
                else return 1;
            };
        }
        if (length < 2) return this;
        return merge(this.slice(0, middle).msort(compare), this.slice(middle, length).msort(compare), compare);
    }

    //merge two lists


    function merge(left, right, compare) {
        var result = [];
        while (left.length > 0 || right.length > 0) {
            if (left.length > 0 && right.length > 0) {
                if (compare(left[0], right[0]) <= 0) {
                    result.push(left[0]);
                    left = left.slice(1);
                } else {
                    result.push(right[0]);
                    right = right.slice(1);
                }
            } else if (left.length > 0) {
                result.push(left[0]);
                left = left.slice(1);
            } else if (right.length > 0) {
                result.push(right[0]);
                right = right.slice(1);
            }
        }
        return result;
    }
})();

/* jQuery sort elements in a list */
jQuery.fn.sortElements = (function() {
    var sort = [].msort;
    return function(comparator, getSortable) {
        getSortable = getSortable ||
        function() {
            return this;
        };
        var placements = this.map(function() {
            var sortElement = getSortable.call(this),
                parentNode = sortElement.parentNode,
                // Since the element itself will change position, we have
                // to have some way of storing its original position in
                // the DOM. The easiest way is to have a 'flag' node:
                nextSibling = parentNode.insertBefore(document.createTextNode(''), sortElement.nextSibling);

            return function() {
                if (parentNode === this) {
                    throw new Error("You can't sort elements if any one is a descendant of another.");
                }

                // Insert before flag:
                parentNode.insertBefore(this, nextSibling);

                // Remove flag:
                parentNode.removeChild(nextSibling);
            };
        });
        return $(sort.call(this, comparator)).each(function(i) {
            placements[i].call(getSortable.call(this));
        });
    };
})();

//JQuery plugin which provides rankingTableUpdate function to animate the update of a table.
//Note: Requires JQuery 1.4.3 and
//        "Bernie's Better Animation" library (http://berniesumption.com/software/files/2010/09/animator.zip)
//
//Author: Mark Rhodes
//Version: 1.0
//Company: ScottLogic
//Date: 17th November 2010
(function($) {

    //Defines the 16 standard html colours by they hash codes - if you use others then
    //don't complain when it doesn't work!
    var standardColorNames = {
        aqua: '#00FFFF',
        black: '#000000',
        blue: '#0000FF',
        fuchsia: '#FF00FF',
        gray: '#808080',
        grey: '#808080',
        lime: '#00FF00',
        maroon: '#800000',
        navy: '#000080',
        olive: '#808000',
        purple: '#800080',
        red: '#FF0000',
        silver: '#C0C0C0',
        teal: '#008080',
        white: '#FFFFFF',
        yellow: '#FFFF00'
    };

    //Simple non-infallable function to obtain the background color of an element.
    //Assumes that the element and parents are statically positioned, not absolute etc.
    //and works by checking the computed style of an element, if this is transparent
    //recursively looks at colour of the parent node, if no colour is found uses white.
    //Also considers all rgba values to be transparent, so don't use them..
    var getColourOfBackground = function(ele) {
        var colorStr = $(ele).css("backgroundColor");
        if (colorStr.indexOf('rgba') === 0 || colorStr === 'transparent') { //works for 'rgba(0,0,0,0)' in Chrome, Safari and 'transparent' for IE, FF, Opera
            return (ele.parentNode != document) ? getColourOfBackground(ele.parentNode) : '#FFFFFF';
        }
        if (colorStr in standardColorNames) {
            colorStr = standardColorNames[colorStr];
        } else if (colorStr.indexOf('#') == -1) { //in case it's already a hex color (occurs in IE).
            colorStr = cssColorToHex(colorStr);
        }
        return colorStr;
    }

    //Convert the rgb value to the hex equivilent..
    var cssColorToHex = function(colorStr) {
        var hex = '#';
        $.each(colorStr.substring(4).split(','), function(i, str) {
            var h = ($.trim(str.replace(')', '')) * 1).toString(16);
            hex += (h.length == 1) ? "0" + h : h;
        });
        return hex;
    };

    var getMinLeftValueInOptions = function(options) {
        var minLeft = 0;
        $.each(options.animationSettings, function(part, settings) {
            minLeft = Math.min(settings.left, minLeft);
        });
        return minLeft;
    }

    //The defalt options to use in the case that none as specified..
    var defaultOptions = {
        onComplete: function() { /*do nothing*/
        },
        duration: [1000, 0, 700, 0, 500],
        //The milliseconds to do each phase and the deplay between them
        extractIdFromCell: function(td) {
            return $.trim($(td).html());
        },
        //the function to use to extract the id value from a cell in the id column.
        animationSettings: {
            up: {
                left: -25,
                // Move left
                backgroundColor: '#aef5ae' // Dullish green  '#004400'
            },
            down: {
                left: 25,
                // Move right
                backgroundColor: '#fa8482' // Dullish red '#550000'
            },
            fresh: {
                left: 0,
                //Stay put in first stage.
                backgroundColor: '#FFFF33' // Yellow
            },
            drop: {
                left: 0,
                //Stay put in first stage.
                backgroundColor: '#550055' // Purple
            }
        }
    };

    //given a cell it removes the padding from it and returns what it was as a string.
    var removeAndReturnPadding = function(td) {
        td = $(td);
        var cellPadding = td.css("padding-top") + " " + td.css("padding-right") + " " + td.css("padding-bottom") + " " + td.css("padding-left");
        td.css({
            padding: 0
        });
        return cellPadding;
    };

    //should be given the options passed in from the command-line and
    //fills in any missing parameters with default values.
    var completeOptions = function(options) {
        if (!options) {
            options = {};
        }
        //Allow some parameters to be given as single values that can be converted to what the
        //rankingTableUpdate function expects..
        if (typeof options.duration === 'number') {
            var overThree = options.duration / 3;
            //set each phase to take a third of the time with no delay between them..
            options.duration = [overThree, 0, overThree, 0, overThree];
        }

        //set any unset parameters to the default values..
        return $.extend(true, {}, defaultOptions, options);
    }

    //Replaces the first element is "this" jQuery object, which should be an HTML table,
    //with the given new version and animates the changes based on the given options.
    //
    //params:
    //    newTable - an HTML table element or jQuery object in which the first element is
    //             an HTML table
    //  options - a JS object which defines how the animation should operate.
    $.fn.rankingTableUpdate = function(newTable, options) {
        //make sure we have jQuery wrapped versions of both tables..
        var jOrigTable = this,
            jNewTable = $(newTable);

        //store a reference to the actual tables that will be updated..
        var origTable = this[0];
        newTable = jNewTable[0];

        //we need the new table to be hidden and have the same parent as the orig table,
        //so we can measure it and get colour values from it accurately..
        jNewTable.hide();
        var jOrigTableParent = jOrigTable.parent();
        if (jNewTable.parent()[0] !== jOrigTableParent[0]) {
            jOrigTableParent.append(newTable);
        }

        //fills in any blank options will default values so as to simplify this function's code..
        options = completeOptions(options);

        //store references to the tbodies and "cache" some values..
        var origTBody = origTable.tBodies[0];
        var newTBody = newTable.tBodies[0];
        var rowsInNewTable = newTBody.rows.length; //cache this as it's slow in IE.
        var columnsInEachRow = origTable.tHead.rows[0].cells.length;
        var colourBehindTable = getColourOfBackground(origTable.parentNode);

        //both tables should have the same columns, we need to examine
        //these and figure out which columns need to be updated..
        //columns should have class either anim:position, anim:constant, anim:id, or anim:update.
        var idColumn = 0,
            positionColumns = new Array();
        constantColumns = {}, updatingColumns = {}, noUpdatingColumns = true;
        $(origTable.tHead.rows[0].cells).each(function(i, td) {
            td = $(td);
            if (td.hasClass("anim:position")) {
                positionColumns.push(i);
                updatingColumns[i] = true;
            } else if (td.hasClass("anim:id")) {
                idColumn = i;
                constantColumns[i] = true;
            } else if (td.hasClass("anim:constant")) {
                constantColumns[i] = true;
            } else { //by default treat as an updating column..
                updatingColumns[i] = true;
                noUpdatingColumns = false;
            }
        });

        //associate the value of the id column for the table with the row in which is appears..
        var origTableIdsToRows = {},
            newTableIdsToRows = {};
        $(origTBody.rows).each(function(row, tr) {
            origTableIdsToRows[options.extractIdFromCell(tr.cells[idColumn])] = row;
        });
        $(newTBody.rows).each(function(row, tr) {
            newTableIdsToRows[options.extractIdFromCell(tr.cells[idColumn])] = row;
        });

        //break the id's in five sets - up, down, fresh, drop, stayPut
        var up = {},
            down = {},
            fresh = {},
            drop = {},
            stayPut = {};
        var maxRowsUp = 0,
            maxRowsDown = 0,
            numRowsStaying = 0;
        $.each(origTableIdsToRows, function(id, oldRow) {
            //case that a the row needs to be moved..
            if (id in newTableIdsToRows) {
                var newRow = newTableIdsToRows[id];
                var diff = oldRow - newRow;
                if (diff > 0) {
                    up[oldRow] = newRow;
                    maxRowsUp = Math.max(diff, maxRowsUp);
                } else if (diff < 0) {
                    down[oldRow] = newRow;
                    maxRowsDown = Math.max(0 - maxRowsDown, maxRowsDown)
                } else {
                    stayPut[oldRow] = true;
                    numRowsStaying++;
                }
                delete newTableIdsToRows[id];
            } else {
                drop[oldRow] = true;
            }
        });
        //elements left in the new table must be new ones..
        $.each(newTableIdsToRows, function(id, newRow) {
            //need to make the new unique from anything in the other objects..
            fresh['x' + newRow] = true;
        });

        //don't bother doing anything if all rows are staying put and no columns are updating..
        if (numRowsStaying === rowsInNewTable && noUpdatingColumns) {
            //wait a little while then do it (in case program is expecting it to take sometime..
            setTimeout(function() {
                //perform the actual swap
                jOrigTable.replaceWith(jNewTable);
                jNewTable.show();

                //run the onComplete callback function..
                options.onComplete();
            }, 10);
            return;
        }

        //--- Animation setup ------
        //we need to store the heights of the tables so that we can animate any differences between them..
        var origHeight = jOrigTable.height();
        var newHeight = jNewTable.height();
        var minLeftValue = getMinLeftValueInOptions(options);

        //we first wrap the table in a wrapper div that will hide any extra rows we add to it
        //A bit of trickery is required here, to move the table to the right the first, then move
        //the wrapper to the left the same amount, this is because setting overflow: hidden or (even just
        //for overflow-y: hidden!) prevents an inner element extending the left hand side of the container.
        jOrigTable.css({
            position: "relative",
            left: 0 - minLeftValue
        });
        jOrigTable.wrap(
        $("<div />", {
            css: {
                height: origHeight,
                overflow: "hidden",
                position: "relative",
                left: minLeftValue
            }
        }));

        //wrap table cell contents with a div..
        $(origTBody.rows).each(function(row, tr) {
            $.each(tr.cells, function(column, td) {
                var wrapper = $('<div/>', {
                    'class': 'moveable',
                    css: {
                        position: "relative",
                        padding: removeAndReturnPadding(td)
                    }
                });
                wrapper.data("row", row);
                wrapper.data("column", column);
                $(td).wrapInner(wrapper);
            });
        });

        var rowDiff = rowsInNewTable - origTBody.rows.length;
        //we'll attach empty extra rows to the end of the table which will be used to hold
        //data latter and will stop fresh rows at the bottom from showing up.
        if (rowDiff > 0) {
            var emptyRow = $('<tr/>');
            //put something in first cell to ensure height is ok.
            emptyRow.append($('<td/>', {
                css: {
                    color: colourBehindTable,
                    backgroundColor: colourBehindTable
                }
            }).html('a'));
            for (var i = 1; i < columnsInEachRow; i++) {
                emptyRow.append($('<td/>', {
                    css: {
                        backgroundColor: colourBehindTable
                    }
                }));
            }
            jOrigTable.append(emptyRow);

            while (rowDiff > 0) {
                //append a clone so that there is an extra empty row in the table..
                var emptyClone = emptyRow.clone();
                jOrigTable.append(emptyClone);
                rowDiff--;
            }
        }

        //Now do the same for the fresh rows, for these we'll:
        //  1. Clone the row in the new table.
        //  2. Attach the clone to the end of the original table
        //  3. Wrap the cells with divs like above
        $.each(fresh, function(row) {
            //the row which the fresh row will move to..
            row = row.substring(1) * 1;
            var clone = $(newTBody.rows[row]).clone();
            jOrigTable.append(clone);
            $(clone[0].cells).each(function(column, td) {
                var wrapper = $('<div />', {
                    'class': 'moveable',
                    css: {
                        position: "relative",
                        padding: removeAndReturnPadding(td),
                        backgroundColor: options.animationSettings.fresh.backgroundColor,
                        left: options.animationSettings.fresh.left
                    }
                });
                //need to make the row unique so that it doesn't clash..
                wrapper.data("row", 'x' + row);
                wrapper.data("column", column);
                $(td).wrapInner(wrapper);
            });
        });

        //Set up a simple animator chain as the AnimatorChain in animator.js seems to be buggy..
        //The third animator should set the table to the state of the new table, this involves showing
        //the new values, shrinking the table if required and pushing the rows left/right so they
        //are back in the table.
        //When it's finished it should perform the switch between the tables.
        var thirdAnimator = new Animator({
            //when it's finished update the table as expected and tidy up..
            onComplete: function() {
                //perform the actual swap (note we replace parentNode which is the table wrapper)..
                $(origTable.parentNode).replaceWith(jNewTable);
                jNewTable.show();

                //run the onComplete callback function..
                options.onComplete();
            },
            duration: options.duration[4]
        });

        //In the second phase the rows should be moved verically to their required positions.
        var secondAnimator = new Animator({
            onComplete: function() {
                //play final phase animation after specified delay..
                setTimeout(function() {
                    thirdAnimator.play();
                }, options.duration[3]);
            },
            duration: options.duration[2]
        });

        //In the intial stage of the animation the updating values should be hidden, the rows coloured
        //and pulled to the left/right as expected and the table extended to accommodate new rows.
        //When complete the values which were hidden are altered to their new ones.
        var updateValue = []; //the divs with the values we'll change to the new values.
        var firstAnimator = new Animator({
            onComplete: function() {
                //Update values should contain pairs, the dom element to update and the new
                //value to use, which is a string which may encode dom elements if required..
                $.each(updateValue, function(i, elemAndValue) {
                    $(elemAndValue[0]).html(elemAndValue[1]);
                });

                //play the second animation after specified delay..
                setTimeout(function() {
                    secondAnimator.play()
                }, options.duration[1]);
            },
            duration: options.duration[0]
        });

        //if we need to make the table bigger do this at the start of the animation..
        if (origHeight < newHeight) {
            firstAnimator.addSubject(new NumericalStyleSubject(origTable.parentNode, "height", origHeight, newHeight));

        } else if (origHeight > newHeight) { //if the table needs to shrink, do this at the end.
            thirdAnimator.addSubject(new NumericalStyleSubject(origTable.parentNode, "height", origHeight, newHeight));
        }

        jOrigTable.find('div.moveable').each(function(i, wrapper) {

            var newCell; //this will be set to the cell in the new table which is equivilent to the one being
            //wrapped, this will be remain null for wrappers in fresh and dropped rows.
            var oldCell = wrapper.parentNode;
            var row = $(wrapper).data("row");
            var column = $(wrapper).data("column");

            //need to fix the colour so that it really looks like the rows are moving out of place..
            if (!(row in stayPut) && !(row in fresh)) {
                var initialBGColor = getColourOfBackground(oldCell);
                $(wrapper).css('backgroundColor', initialBGColor);
                $(wrapper.parentNode).css('backgroundColor', colourBehindTable);
            }

            if (row in up) {
                var animationSetting = options.animationSettings.up;
                var rowsUp = row - up[row];

                var animateToBGColor = animationSetting.backgroundColor;
                var animateLeft = animationSetting.left;
                newCell = newTBody.rows[up[row]].cells[column];

                //move it left/right and change the background color..
                firstAnimator.addSubject(new NumericalStyleSubject(wrapper, "left", 0, animateLeft));
                firstAnimator.addSubject(new ColorStyleSubject(wrapper, "background-color", initialBGColor, animateToBGColor));

                //move the row up..
                var topDiff = $(origTBody.rows[up[row]]).position().top - $(origTBody.rows[row]).position().top;
                secondAnimator.addSubject(new NumericalStyleSubject(wrapper, "top", 0, topDiff));

                //move it back into position and colour the background to the new cell's colour..
                thirdAnimator.addSubject(new NumericalStyleSubject(wrapper, "left", animateLeft, 0));
                thirdAnimator.addSubject(new ColorStyleSubject(wrapper, "background-color", animateToBGColor, getColourOfBackground(newCell)));

            } else if (row in down) {
                var animationSetting = options.animationSettings.down;
                var rowsDown = down[row] - row;

                var animateToBGColor = animationSetting.backgroundColor;
                var animateLeft = animationSetting.left;
                newCell = newTBody.rows[down[row]].cells[column];

                //move it left/right and change the background color..
                firstAnimator.addSubject(new NumericalStyleSubject(wrapper, "left", 0, animateLeft));
                firstAnimator.addSubject(new ColorStyleSubject(wrapper, "background-color", initialBGColor, animateToBGColor));
                $(wrapper.parentNode).css('backgroundColor', colourBehindTable);

                //move the row down..
                var topDiff = $(origTBody.rows[down[row]]).position().top - $(origTBody.rows[row]).position().top;
                secondAnimator.addSubject(new NumericalStyleSubject(wrapper, "top", 0, topDiff));

                //move it back into position and colour the background to the new cell's colour..
                thirdAnimator.addSubject(new NumericalStyleSubject(wrapper, "left", animateLeft, 0));
                thirdAnimator.addSubject(new ColorStyleSubject(wrapper, "background-color", animateToBGColor, getColourOfBackground(newCell)));

            } else if (row in drop) {
                var animationSetting = options.animationSettings.drop;

                var animateToBGColor = animationSetting.backgroundColor;
                var animateLeft = animationSetting.left;
                $(wrapper.parentNode).css('backgroundColor', colourBehindTable);

                //move it left/right and change the background color..
                firstAnimator.addSubject(new NumericalStyleSubject(wrapper, "left", 0, animateLeft));
                firstAnimator.addSubject(new ColorStyleSubject(wrapper, "background-color", initialBGColor, animateToBGColor));

                //move it to the bottom of the table, where it'll be hidden and fade it away.
                var topDiff = newHeight - $(origTBody.rows[row]).position().top;
                secondAnimator.addSubject(new NumericalStyleSubject(wrapper, "top", 0, topDiff));
                secondAnimator.addSubject(new NumericalStyleSubject(wrapper, "opacity", 1, 0, ""));

            } else if (row in fresh) {
                //turn row into a number and lose the preceeding 'x'..
                row = row.substring(1) * 1;

                var animationSetting = options.animationSettings.fresh;
                newCell = newTBody.rows[row].cells[column];

                //move the row up..
                var topDiff = $(origTBody.rows[row]).position().top - $(wrapper.parentNode).position().top;
                secondAnimator.addSubject(new NumericalStyleSubject(wrapper, "top", 0, topDiff));

                //move it back into position and colour the background to the new cell's colour..
                thirdAnimator.addSubject(new ColorStyleSubject(wrapper, "background-color", animationSetting.backgroundColor, getColourOfBackground(newCell)));
                thirdAnimator.addSubject(new NumericalStyleSubject(wrapper, "left", animationSetting.left, 0));

            } else { //must be in stay put..
                newCell = newTBody.rows[row].cells[column];
            }

            //in this case we may need to animate the updating of the value..
            if (column in updatingColumns && (($.inArray(column, positionColumns) != -1) || !(row in stayPut))) {

                //need to inner wrapper which will allow content of cell to be removed..
                $(wrapper).wrapInner($('<div />', {
                    'class': 'innerWrapper'
                }));
                var innerWrapper = $(wrapper).find(".innerWrapper")[0]; //note: this seems like excessive work but there
                //seems to be a bug with jQuery requiring it to be like this!
                firstAnimator.addSubject(new NumericalStyleSubject(innerWrapper, "opacity", 1, 0, ""));
                if (newCell != null) {
                    thirdAnimator.addSubject(new NumericalStyleSubject(innerWrapper, "opacity", 0, 1, ""));
                    updateValue.push([innerWrapper, $(newCell).html()]);
                }
            }
        });

        //trigger the animation..
        firstAnimator.play();

        //make it chainable..
        return this;
    }

})(jQuery);


////////////////////////////////////////////////// ANIMATOR LIBRARY /////////////////////////////
/*  
    Animator.js 1.1.9
    
    This library is released under the BSD license:

    Copyright (c) 2006, Bernard Sumption. All rights reserved.
    
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:
    
    Redistributions of source code must retain the above copyright notice, this
    list of conditions and the following disclaimer. Redistributions in binary
    form must reproduce the above copyright notice, this list of conditions and
    the following disclaimer in the documentation and/or other materials
    provided with the distribution. Neither the name BernieCode nor
    the names of its contributors may be used to endorse or promote products
    derived from this software without specific prior written permission. 
    
    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
    ARE DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE FOR
    ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
    LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
    OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
    DAMAGE.

*/


// Applies a sequence of numbers between 0 and 1 to a number of subjects
// construct - see setOptions for parameters


function Animator(options) {
    this.setOptions(options);
    var _this = this;
    this.timerDelegate = function() {
        _this.onTimerEvent()
    };
    this.subjects = [];
    this.target = 0;
    this.state = 0;
    this.lastTime = null;
};
Animator.prototype = {
    // apply defaults
    setOptions: function(options) {
        this.options = Animator.applyDefaults({
            interval: 20,
            // time between animation frames
            duration: 400,
            // length of animation
            onComplete: function() {},
            onStep: function() {},
            transition: Animator.tx.easeInOut
        }, options);
    },
    // animate from the current state to provided value
    seekTo: function(to) {
        this.seekFromTo(this.state, to);
    },
    // animate from the current state to provided value
    seekFromTo: function(from, to) {
        this.target = Math.max(0, Math.min(1, to));
        this.state = Math.max(0, Math.min(1, from));
        this.lastTime = new Date().getTime();
        if (!this.intervalId) {
            this.intervalId = window.setInterval(this.timerDelegate, this.options.interval);
        }
    },
    // animate from the current state to provided value
    jumpTo: function(to) {
        this.target = this.state = Math.max(0, Math.min(1, to));
        this.propagate();
    },
    // seek to the opposite of the current target
    toggle: function() {
        this.seekTo(1 - this.target);
    },
    // add a function or an object with a method setState(state) that will be called with a number
    // between 0 and 1 on each frame of the animation
    addSubject: function(subject) {
        this.subjects[this.subjects.length] = subject;
        return this;
    },
    // remove all subjects
    clearSubjects: function() {
        this.subjects = [];
    },
    // forward the current state to the animation subjects
    propagate: function() {
        var value = this.options.transition(this.state);
        for (var i = 0; i < this.subjects.length; i++) {
            if (this.subjects[i].setState) {
                this.subjects[i].setState(value);
            } else {
                this.subjects[i](value);
            }
        }
    },
    // called once per frame to update the current state
    onTimerEvent: function() {
        var now = new Date().getTime();
        var timePassed = now - this.lastTime;
        this.lastTime = now;
        var movement = (timePassed / this.options.duration) * (this.state < this.target ? 1 : -1);
        if (Math.abs(movement) >= Math.abs(this.state - this.target)) {
            this.state = this.target;
        } else {
            this.state += movement;
        }

        try {
            this.propagate();
        } finally {
            this.options.onStep.call(this);
            if (this.target == this.state) {
                window.clearInterval(this.intervalId);
                this.intervalId = null;
                this.options.onComplete.call(this);
            }
        }
    },
    // shortcuts
    play: function() {
        this.seekFromTo(0, 1)
    },
    reverse: function() {
        this.seekFromTo(1, 0)
    },
    // return a string describing this Animator, for debugging
    inspect: function() {
        var str = "#<Animator:\n";
        for (var i = 0; i < this.subjects.length; i++) {
            str += this.subjects[i].inspect();
        }
        str += ">";
        return str;
    }
}
// merge the properties of two objects
Animator.applyDefaults = function(defaults, prefs) {
    prefs = prefs || {};
    var prop, result = {};
    for (prop in defaults) result[prop] = prefs[prop] !== undefined ? prefs[prop] : defaults[prop];
    return result;
}
// make an array from any object
Animator.makeArray = function(o) {
    if (o == null) return [];
    if (!o.length) return [o];
    var result = [];
    for (var i = 0; i < o.length; i++) result[i] = o[i];
    return result;
}
// convert a dash-delimited-property to a camelCaseProperty (c/o Prototype, thanks Sam!)
Animator.camelize = function(string) {
    var oStringList = string.split('-');
    if (oStringList.length == 1) return oStringList[0];

    var camelizedString = string.indexOf('-') == 0 ? oStringList[0].charAt(0).toUpperCase() + oStringList[0].substring(1) : oStringList[0];

    for (var i = 1, len = oStringList.length; i < len; i++) {
        var s = oStringList[i];
        camelizedString += s.charAt(0).toUpperCase() + s.substring(1);
    }
    return camelizedString;
}
// syntactic sugar for creating CSSStyleSubjects
Animator.apply = function(el, style, options) {
    if (style instanceof Array) {
        return new Animator(options).addSubject(new CSSStyleSubject(el, style[0], style[1]));
    }
    return new Animator(options).addSubject(new CSSStyleSubject(el, style));
}
// make a transition function that gradually accelerates. pass a=1 for smooth
// gravitational acceleration, higher values for an exaggerated effect
Animator.makeEaseIn = function(a) {
    return function(state) {
        return Math.pow(state, a * 2);
    }
}
// as makeEaseIn but for deceleration
Animator.makeEaseOut = function(a) {
    return function(state) {
        return 1 - Math.pow(1 - state, a * 2);
    }
}
// make a transition function that, like an object with momentum being attracted to a point,
// goes past the target then returns
Animator.makeElastic = function(bounces) {
    return function(state) {
        state = Animator.tx.easeInOut(state);
        return ((1 - Math.cos(state * Math.PI * bounces)) * (1 - state)) + state;
    }
}
// make an Attack Decay Sustain Release envelope that starts and finishes on the same level
// 
Animator.makeADSR = function(attackEnd, decayEnd, sustainEnd, sustainLevel) {
    if (sustainLevel == null) sustainLevel = 0.5;
    return function(state) {
        if (state < attackEnd) {
            return state / attackEnd;
        }
        if (state < decayEnd) {
            return 1 - ((state - attackEnd) / (decayEnd - attackEnd) * (1 - sustainLevel));
        }
        if (state < sustainEnd) {
            return sustainLevel;
        }
        return sustainLevel * (1 - ((state - sustainEnd) / (1 - sustainEnd)));
    }
}
// make a transition function that, like a ball falling to floor, reaches the target and/
// bounces back again
Animator.makeBounce = function(bounces) {
    var fn = Animator.makeElastic(bounces);
    return function(state) {
        state = fn(state);
        return state <= 1 ? state : 2 - state;
    }
}

// pre-made transition functions to use with the 'transition' option
Animator.tx = {
    easeInOut: function(pos) {
        return ((-Math.cos(pos * Math.PI) / 2) + 0.5);
    },
    linear: function(x) {
        return x;
    },
    easeIn: Animator.makeEaseIn(1.5),
    easeOut: Animator.makeEaseOut(1.5),
    strongEaseIn: Animator.makeEaseIn(2.5),
    strongEaseOut: Animator.makeEaseOut(2.5),
    elastic: Animator.makeElastic(1),
    veryElastic: Animator.makeElastic(3),
    bouncy: Animator.makeBounce(1),
    veryBouncy: Animator.makeBounce(3)
}

// animates a pixel-based style property between two integer values


function NumericalStyleSubject(els, property, from, to, units) {
    this.els = Animator.makeArray(els);
    if (property == 'opacity' && window.ActiveXObject) {
        this.property = 'filter';
    } else {
        this.property = Animator.camelize(property);
    }
    this.from = parseFloat(from);
    this.to = parseFloat(to);
    this.units = units != null ? units : 'px';
}
NumericalStyleSubject.prototype = {
    setState: function(state) {
        var style = this.getStyle(state);
        var visibility = (this.property == 'opacity' && state == 0) ? 'hidden' : '';
        var j = 0;
        for (var i = 0; i < this.els.length; i++) {
            try {
                this.els[i].style[this.property] = style;
            } catch (e) {
                // ignore fontWeight - intermediate numerical values cause exeptions in firefox
                if (this.property != 'fontWeight') throw e;
            }
            if (j++ > 20) return;
        }
    },
    getStyle: function(state) {
        state = this.from + ((this.to - this.from) * state);
        if (this.property == 'filter') return "alpha(opacity=" + Math.round(state * 100) + ")";
        if (this.property == 'opacity') return state;
        return Math.round(state) + this.units;
    },
    inspect: function() {
        return "\t" + this.property + "(" + this.from + this.units + " to " + this.to + this.units + ")\n";
    }
}

// animates a colour based style property between two hex values


function ColorStyleSubject(els, property, from, to) {
    this.els = Animator.makeArray(els);
    this.property = Animator.camelize(property);
    this.to = this.expandColor(to);
    this.from = this.expandColor(from);
    this.origFrom = from;
    this.origTo = to;
}

ColorStyleSubject.prototype = {
    // parse "#FFFF00" to [256, 256, 0]
    expandColor: function(color) {
        var hexColor, red, green, blue;
        hexColor = ColorStyleSubject.parseColor(color);
        if (hexColor) {
            red = parseInt(hexColor.slice(1, 3), 16);
            green = parseInt(hexColor.slice(3, 5), 16);
            blue = parseInt(hexColor.slice(5, 7), 16);
            return [red, green, blue]
        }
        if (window.DEBUG) {
            alert("Invalid colour: '" + color + "'");
        }
    },
    getValueForState: function(color, state) {
        return Math.round(this.from[color] + ((this.to[color] - this.from[color]) * state));
    },
    setState: function(state) {
        var color = '#' + ColorStyleSubject.toColorPart(this.getValueForState(0, state)) + ColorStyleSubject.toColorPart(this.getValueForState(1, state)) + ColorStyleSubject.toColorPart(this.getValueForState(2, state));
        for (var i = 0; i < this.els.length; i++) {
            this.els[i].style[this.property] = color;
        }
    },
    inspect: function() {
        return "\t" + this.property + "(" + this.origFrom + " to " + this.origTo + ")\n";
    }
}

// return a properly formatted 6-digit hex colour spec, or false
ColorStyleSubject.parseColor = function(string) {
    var color = '#',
        match;
    if (match = ColorStyleSubject.parseColor.rgbRe.exec(string)) {
        var part;
        for (var i = 1; i <= 3; i++) {
            part = Math.max(0, Math.min(255, parseInt(match[i])));
            color += ColorStyleSubject.toColorPart(part);
        }
        return color;
    }
    if (match = ColorStyleSubject.parseColor.hexRe.exec(string)) {
        if (match[1].length == 3) {
            for (var i = 0; i < 3; i++) {
                color += match[1].charAt(i) + match[1].charAt(i);
            }
            return color;
        }
        return '#' + match[1];
    }
    return false;
}
// convert a number to a 2 digit hex string
ColorStyleSubject.toColorPart = function(number) {
    if (number > 255) number = 255;
    var digits = number.toString(16);
    if (number < 16) return '0' + digits;
    return digits;
}
ColorStyleSubject.parseColor.rgbRe = /^rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$/i;
ColorStyleSubject.parseColor.hexRe = /^\#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/;

// Animates discrete styles, i.e. ones that do not scale but have discrete values
// that can't be interpolated


function DiscreteStyleSubject(els, property, from, to, threshold) {
    this.els = Animator.makeArray(els);
    this.property = Animator.camelize(property);
    this.from = from;
    this.to = to;
    this.threshold = threshold || 0.5;
}

DiscreteStyleSubject.prototype = {
    setState: function(state) {
        var j = 0;
        for (var i = 0; i < this.els.length; i++) {
            this.els[i].style[this.property] = state <= this.threshold ? this.from : this.to;
        }
    },
    inspect: function() {
        return "\t" + this.property + "(" + this.from + " to " + this.to + " @ " + this.threshold + ")\n";
    }
}

// animates between two styles defined using CSS.
// if style1 and style2 are present, animate between them, if only style1
// is present, animate between the element's current style and style1


function CSSStyleSubject(els, style1, style2) {
    els = Animator.makeArray(els);
    this.subjects = [];
    if (els.length == 0) return;
    var prop, toStyle, fromStyle;
    if (style2) {
        fromStyle = this.parseStyle(style1, els[0]);
        toStyle = this.parseStyle(style2, els[0]);
    } else {
        toStyle = this.parseStyle(style1, els[0]);
        fromStyle = {};
        for (prop in toStyle) {
            fromStyle[prop] = CSSStyleSubject.getStyle(els[0], prop);
        }
    }
    // remove unchanging properties
    var prop;
    for (prop in fromStyle) {
        if (fromStyle[prop] == toStyle[prop]) {
            delete fromStyle[prop];
            delete toStyle[prop];
        }
    }
    // discover the type (numerical or colour) of each style
    var prop, units, match, type, from, to;
    for (prop in fromStyle) {
        var fromProp = String(fromStyle[prop]);
        var toProp = String(toStyle[prop]);
        if (toStyle[prop] == null) {
            if (window.DEBUG) alert("No to style provided for '" + prop + '"');
            continue;
        }

        if (from = ColorStyleSubject.parseColor(fromProp)) {
            to = ColorStyleSubject.parseColor(toProp);
            type = ColorStyleSubject;
        } else if (fromProp.match(CSSStyleSubject.numericalRe) && toProp.match(CSSStyleSubject.numericalRe)) {
            from = parseFloat(fromProp);
            to = parseFloat(toProp);
            type = NumericalStyleSubject;
            match = CSSStyleSubject.numericalRe.exec(fromProp);
            var reResult = CSSStyleSubject.numericalRe.exec(toProp);
            if (match[1] != null) {
                units = match[1];
            } else if (reResult[1] != null) {
                units = reResult[1];
            } else {
                units = reResult;
            }
        } else if (fromProp.match(CSSStyleSubject.discreteRe) && toProp.match(CSSStyleSubject.discreteRe)) {
            from = fromProp;
            to = toProp;
            type = DiscreteStyleSubject;
            units = 0; // hack - how to get an animator option down to here
        } else {
            if (window.DEBUG) {
                alert("Unrecognised format for value of " + prop + ": '" + fromStyle[prop] + "'");
            }
            continue;
        }
        this.subjects[this.subjects.length] = new type(els, prop, from, to, units);
    }
}

CSSStyleSubject.prototype = {
    // parses "width: 400px; color: #FFBB2E" to {width: "400px", color: "#FFBB2E"}
    parseStyle: function(style, el) {
        var rtn = {};
        // if style is a rule set
        if (style.indexOf(":") != -1) {
            var styles = style.split(";");
            for (var i = 0; i < styles.length; i++) {
                var parts = CSSStyleSubject.ruleRe.exec(styles[i]);
                if (parts) {
                    rtn[parts[1]] = parts[2];
                }
            }
        }
        // else assume style is a class name
        else {
            var prop, value, oldClass;
            oldClass = el.className;
            el.className = style;
            for (var i = 0; i < CSSStyleSubject.cssProperties.length; i++) {
                prop = CSSStyleSubject.cssProperties[i];
                value = CSSStyleSubject.getStyle(el, prop);
                if (value != null) {
                    rtn[prop] = value;
                }
            }
            el.className = oldClass;
        }
        return rtn;

    },
    setState: function(state) {
        for (var i = 0; i < this.subjects.length; i++) {
            this.subjects[i].setState(state);
        }
    },
    inspect: function() {
        var str = "";
        for (var i = 0; i < this.subjects.length; i++) {
            str += this.subjects[i].inspect();
        }
        return str;
    }
}
// get the current value of a css property, 
CSSStyleSubject.getStyle = function(el, property) {
    var style;
    if (document.defaultView && document.defaultView.getComputedStyle) {
        style = document.defaultView.getComputedStyle(el, "").getPropertyValue(property);
        if (style) {
            return style;
        }
    }
    property = Animator.camelize(property);
    if (el.currentStyle) {
        style = el.currentStyle[property];
    }
    return style || el.style[property]
}


CSSStyleSubject.ruleRe = /^\s*([a-zA-Z\-]+)\s*:\s*(\S(.+\S)?)\s*$/;
CSSStyleSubject.numericalRe = /^-?\d+(?:\.\d+)?(%|[a-zA-Z]{2})?$/;
CSSStyleSubject.discreteRe = /^\w+$/;

// required because the style object of elements isn't enumerable in Safari
/*
CSSStyleSubject.cssProperties = ['background-color','border','border-color','border-spacing',
'border-style','border-top','border-right','border-bottom','border-left','border-top-color',
'border-right-color','border-bottom-color','border-left-color','border-top-width','border-right-width',
'border-bottom-width','border-left-width','border-width','bottom','color','font-size','font-size-adjust',
'font-stretch','font-style','height','left','letter-spacing','line-height','margin','margin-top',
'margin-right','margin-bottom','margin-left','marker-offset','max-height','max-width','min-height',
'min-width','orphans','outline','outline-color','outline-style','outline-width','overflow','padding',
'padding-top','padding-right','padding-bottom','padding-left','quotes','right','size','text-indent',
'top','width','word-spacing','z-index','opacity','outline-offset'];*/


CSSStyleSubject.cssProperties = ['azimuth', 'background', 'background-attachment', 'background-color', 'background-image', 'background-position', 'background-repeat', 'border-collapse', 'border-color', 'border-spacing', 'border-style', 'border-top', 'border-top-color', 'border-right-color', 'border-bottom-color', 'border-left-color', 'border-top-style', 'border-right-style', 'border-bottom-style', 'border-left-style', 'border-top-width', 'border-right-width', 'border-bottom-width', 'border-left-width', 'border-width', 'bottom', 'clear', 'clip', 'color', 'content', 'cursor', 'direction', 'display', 'elevation', 'empty-cells', 'css-float', 'font', 'font-family', 'font-size', 'font-size-adjust', 'font-stretch', 'font-style', 'font-variant', 'font-weight', 'height', 'left', 'letter-spacing', 'line-height', 'list-style', 'list-style-image', 'list-style-position', 'list-style-type', 'margin', 'margin-top', 'margin-right', 'margin-bottom', 'margin-left', 'max-height', 'max-width', 'min-height', 'min-width', 'orphans', 'outline', 'outline-color', 'outline-style', 'outline-width', 'overflow', 'padding', 'padding-top', 'padding-right', 'padding-bottom', 'padding-left', 'pause', 'position', 'right', 'size', 'table-layout', 'text-align', 'text-decoration', 'text-indent', 'text-shadow', 'text-transform', 'top', 'vertical-align', 'visibility', 'white-space', 'width', 'word-spacing', 'z-index', 'opacity', 'outline-offset', 'overflow-x', 'overflow-y'];


// chains several Animator objects together


function AnimatorChain(animators, options) {
    this.animators = animators;
    this.setOptions(options);
    for (var i = 0; i < this.animators.length; i++) {
        this.listenTo(this.animators[i]);
    }
    this.forwards = false;
    this.current = 0;
}

AnimatorChain.prototype = {
    // apply defaults
    setOptions: function(options) {
        this.options = Animator.applyDefaults({
            // by default, each call to AnimatorChain.play() calls jumpTo(0) of each animator
            // before playing, which can cause flickering if you have multiple animators all
            // targeting the same element. Set this to false to avoid this.
            resetOnPlay: true
        }, options);
    },
    // play each animator in turn
    play: function() {
        this.forwards = true;
        this.current = -1;
        if (this.options.resetOnPlay) {
            for (var i = 0; i < this.animators.length; i++) {
                this.animators[i].jumpTo(0);
            }
        }
        this.advance();
    },
    // play all animators backwards
    reverse: function() {
        this.forwards = false;
        this.current = this.animators.length;
        if (this.options.resetOnPlay) {
            for (var i = 0; i < this.animators.length; i++) {
                this.animators[i].jumpTo(1);
            }
        }
        this.advance();
    },
    // if we have just play()'d, then call reverse(), and vice versa
    toggle: function() {
        if (this.forwards) {
            this.seekTo(0);
        } else {
            this.seekTo(1);
        }
    },
    // internal: install an event listener on an animator's onComplete option
    // to trigger the next animator
    listenTo: function(animator) {
        var oldOnComplete = animator.options.onComplete;
        var _this = this;
        animator.options.onComplete = function() {
            if (oldOnComplete) oldOnComplete.call(animator);
            _this.advance();
        }
    },
    // play the next animator
    advance: function() {
        if (this.forwards) {
            if (this.animators[this.current + 1] == null) return;
            this.current++;
            this.animators[this.current].play();
        } else {
            if (this.animators[this.current - 1] == null) return;
            this.current--;
            this.animators[this.current].reverse();
        }
    },
    // this function is provided for drop-in compatibility with Animator objects,
    // but only accepts 0 and 1 as target values
    seekTo: function(target) {
        if (target <= 0) {
            this.forwards = false;
            this.animators[this.current].seekTo(0);
        } else {
            this.forwards = true;
            this.animators[this.current].seekTo(1);
        }
    }
}

// an Accordion is a class that creates and controls a number of Animators. An array of elements is passed in,
// and for each element an Animator and a activator button is created. When an Animator's activator button is
// clicked, the Animator and all before it seek to 0, and all Animators after it seek to 1. This can be used to
// create the classic Accordion effect, hence the name.
// see setOptions for arguments


function Accordion(options) {
    this.setOptions(options);
    var selected = this.options.initialSection,
        current;
    if (this.options.rememberance) {
        current = document.location.hash.substring(1);
    }
    this.rememberanceTexts = [];
    this.ans = [];
    var _this = this;
    for (var i = 0; i < this.options.sections.length; i++) {
        var el = this.options.sections[i];
        var an = new Animator(this.options.animatorOptions);
        var from = this.options.from + (this.options.shift * i);
        var to = this.options.to + (this.options.shift * i);
        an.addSubject(new NumericalStyleSubject(el, this.options.property, from, to, this.options.units));
        an.jumpTo(0);
        var activator = this.options.getActivator(el);
        activator.index = i;
        activator.onclick = function() {
            _this.show(this.index)
        };
        this.ans[this.ans.length] = an;
        this.rememberanceTexts[i] = activator.innerHTML.replace(/\s/g, "");
        if (this.rememberanceTexts[i] === current) {
            selected = i;
        }
    }
    this.show(selected);
}

Accordion.prototype = {
    // apply defaults
    setOptions: function(options) {
        this.options = Object.extend({
            // REQUIRED: an array of elements to use as the accordion sections
            sections: null,
            // a function that locates an activator button element given a section element.
            // by default it takes a button id from the section's "activator" attibute
            getActivator: function(el) {
                return document.getElementById(el.getAttribute("activator"))
            },
            // shifts each animator's range, for example with options {from:0,to:100,shift:20}
            // the animators' ranges will be 0-100, 20-120, 40-140 etc.
            shift: 0,
            // the first page to show
            initialSection: 0,
            // if set to true, document.location.hash will be used to preserve the open section across page reloads 
            rememberance: true,
            // constructor arguments to the Animator objects
            animatorOptions: {}
        }, options || {});
    },
    show: function(section) {
        for (var i = 0; i < this.ans.length; i++) {
            this.ans[i].seekTo(i > section ? 1 : 0);
        }
        if (this.options.rememberance) {
            document.location.hash = this.rememberanceTexts[section];
        }
    }
}