(function () {
    'use strict';

    // --- Configuration ---
    var ARRAY = [2, 5, 8, 12, 16, 23, 38, 42, 56, 72, 91];
    var target = 72;
    var steps = [];
    var currentStep = 0;

    // --- DOM refs ---
    var arrayContainer = document.getElementById('array-container');
    var pointerRow = document.getElementById('pointer-row');
    var stepLabel = document.getElementById('step-label');
    var eli5Text = document.getElementById('eli5-text');
    var targetDisplay = document.getElementById('target-display');
    var btnPrev = document.getElementById('btn-prev');
    var btnNext = document.getElementById('btn-next');
    var btnReset = document.getElementById('btn-reset');

    // --- Step precomputation ---
    function precomputeSteps(arr, tgt) {
        var result = [];
        var left = 0;
        var right = arr.length - 1;
        var eliminated = {};

        // Step 0: initial state
        result.push({
            left: left,
            right: right,
            mid: null,
            eliminated: copySet(eliminated),
            found: false,
            explanation: 'searching for ' + tgt + ' in a sorted array of ' + arr.length + ' elements. left=' + left + ', right=' + right + '.'
        });

        while (left <= right) {
            var mid = Math.floor((left + right) / 2);

            if (arr[mid] === tgt) {
                result.push({
                    left: left,
                    right: right,
                    mid: mid,
                    eliminated: copySet(eliminated),
                    found: true,
                    explanation: 'mid=' + mid + ', arr[' + mid + ']=' + arr[mid] + '. found it! ' + tgt + ' is at index ' + mid + '.'
                });
                break;
            } else if (arr[mid] < tgt) {
                for (var i = left; i <= mid; i++) eliminated[i] = true;
                result.push({
                    left: left,
                    right: right,
                    mid: mid,
                    eliminated: copySet(eliminated),
                    found: false,
                    explanation: 'mid=' + mid + ', arr[' + mid + ']=' + arr[mid] + '. ' + arr[mid] + ' < ' + tgt + ', target is in the RIGHT half. eliminate indices ' + left + '-' + mid + '.'
                });
                left = mid + 1;
            } else {
                for (var j = mid; j <= right; j++) eliminated[j] = true;
                result.push({
                    left: left,
                    right: right,
                    mid: mid,
                    eliminated: copySet(eliminated),
                    found: false,
                    explanation: 'mid=' + mid + ', arr[' + mid + ']=' + arr[mid] + '. ' + arr[mid] + ' > ' + tgt + ', target is in the LEFT half. eliminate indices ' + mid + '-' + right + '.'
                });
                right = mid - 1;
            }
        }

        // If not found (left > right)
        if (result.length > 0 && !result[result.length - 1].found && left > right) {
            result.push({
                left: left,
                right: right,
                mid: null,
                eliminated: copySet(eliminated),
                found: false,
                explanation: 'left > right. search space is empty. ' + tgt + ' is not in the array.'
            });
        }

        return result;
    }

    function copySet(obj) {
        var copy = {};
        for (var k in obj) {
            if (obj.hasOwnProperty(k)) copy[k] = true;
        }
        return copy;
    }

    // --- DOM building ---
    function buildArrayDOM() {
        arrayContainer.innerHTML = '';
        pointerRow.innerHTML = '';

        for (var i = 0; i < ARRAY.length; i++) {
            // Cell
            var cell = document.createElement('div');
            cell.className = 'cell';
            cell.setAttribute('data-index', i);

            var val = document.createElement('div');
            val.className = 'cell-value';
            val.textContent = ARRAY[i];

            var idx = document.createElement('div');
            idx.className = 'cell-index';
            idx.textContent = i;

            cell.appendChild(val);
            cell.appendChild(idx);
            arrayContainer.appendChild(cell);

            // Pointer placeholder
            var ptr = document.createElement('div');
            ptr.className = 'pointer-cell';
            ptr.setAttribute('data-index', i);
            pointerRow.appendChild(ptr);
        }

        // Attach click handlers
        var cells = arrayContainer.querySelectorAll('.cell');
        for (var j = 0; j < cells.length; j++) {
            cells[j].addEventListener('click', handleCellClick);
        }
    }

    function handleCellClick(e) {
        var cell = e.currentTarget;
        var index = parseInt(cell.getAttribute('data-index'), 10);
        target = ARRAY[index];
        targetDisplay.textContent = target;
        steps = precomputeSteps(ARRAY, target);
        currentStep = 0;
        renderStep(currentStep);
    }

    // --- Rendering ---
    function renderStep(stepIndex) {
        var step = steps[stepIndex];

        // Step counter
        stepLabel.textContent = 'step ' + stepIndex + ' of ' + (steps.length - 1);

        // ELI5 text
        eli5Text.textContent = step.explanation;

        // Update cells
        var cells = arrayContainer.querySelectorAll('.cell');
        for (var i = 0; i < cells.length; i++) {
            cells[i].className = 'cell';

            if (step.eliminated[i]) {
                cells[i].classList.add('cell-eliminated');
            } else if (step.found && i === step.mid) {
                cells[i].classList.add('cell-found');
            } else if (i === step.mid) {
                cells[i].classList.add('cell-mid');
            } else {
                cells[i].classList.add('cell-active');
            }
        }

        // Update pointers
        renderPointers(step);

        // Button states
        btnPrev.disabled = (stepIndex === 0);
        btnNext.disabled = (stepIndex === steps.length - 1);
    }

    function renderPointers(step) {
        var ptrs = pointerRow.querySelectorAll('.pointer-cell');
        for (var i = 0; i < ptrs.length; i++) {
            ptrs[i].textContent = '';
            ptrs[i].className = 'pointer-cell';
        }

        if (step.mid === null && step.left !== undefined && step.right !== undefined && step.left <= step.right) {
            // Initial state: show L and R only
            if (step.left >= 0 && step.left < ptrs.length) {
                appendPointerLabel(ptrs[step.left], 'L', 'pointer-L');
            }
            if (step.right >= 0 && step.right < ptrs.length) {
                appendPointerLabel(ptrs[step.right], 'R', 'pointer-R');
            }
            return;
        }

        if (step.mid === null) return;

        // Collect pointers per index
        var pointers = {};
        if (step.left >= 0 && step.left < ARRAY.length && !step.eliminated[step.left]) {
            pointers[step.left] = pointers[step.left] || [];
            pointers[step.left].push({ label: 'L', cls: 'pointer-L' });
        }
        if (step.mid >= 0 && step.mid < ARRAY.length) {
            pointers[step.mid] = pointers[step.mid] || [];
            pointers[step.mid].push({ label: 'M', cls: 'pointer-M' });
        }
        if (step.right >= 0 && step.right < ARRAY.length && !step.eliminated[step.right]) {
            pointers[step.right] = pointers[step.right] || [];
            pointers[step.right].push({ label: 'R', cls: 'pointer-R' });
        }

        for (var idx in pointers) {
            if (pointers.hasOwnProperty(idx)) {
                var i = parseInt(idx, 10);
                if (i >= 0 && i < ptrs.length) {
                    var labels = pointers[i];
                    for (var j = 0; j < labels.length; j++) {
                        appendPointerLabel(ptrs[i], labels[j].label, labels[j].cls);
                    }
                }
            }
        }
    }

    function appendPointerLabel(container, text, cls) {
        if (container.textContent) {
            container.textContent += ',';
        }
        var span = document.createElement('span');
        span.className = cls;
        span.textContent = text;
        container.appendChild(span);
    }

    // --- Event handlers ---
    btnNext.addEventListener('click', function () {
        if (currentStep < steps.length - 1) {
            currentStep++;
            renderStep(currentStep);
        }
    });

    btnPrev.addEventListener('click', function () {
        if (currentStep > 0) {
            currentStep--;
            renderStep(currentStep);
        }
    });

    btnReset.addEventListener('click', function () {
        currentStep = 0;
        renderStep(currentStep);
    });

    document.addEventListener('keydown', function (e) {
        if (e.key === 'ArrowRight' || e.key === ' ') {
            e.preventDefault();
            btnNext.click();
        } else if (e.key === 'ArrowLeft') {
            e.preventDefault();
            btnPrev.click();
        }
    });

    // --- Initialize ---
    buildArrayDOM();
    steps = precomputeSteps(ARRAY, target);
    renderStep(currentStep);

})();
