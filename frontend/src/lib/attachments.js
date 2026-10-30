// @ts-nocheck
import { onDestroy } from "svelte";

export var draggable = (node, params) => {
    let { handleReorder } = params;

    var handleDragStart = (e) => {
        node.dispatchEvent(new PointerEvent('pointerleave'));

        var parent = node.parentNode;
        var currentIndex = Array.from(parent.children).indexOf(node);
        
        e.dataTransfer.effectAllowed = "move";
        e.dataTransfer.setData("text/plain", currentIndex);
        node.classList.add("dragging");
    }

    var handleDragEnd = (e) => {
        node.classList.remove("dragging");
        node.parentNode?.querySelectorAll('.drag-over').forEach(el => el.classList.remove('drag-over'));
    }

    var handleDragOver = (e) => {
        e.preventDefault();
        e.dataTransfer.effectAllowed = "drop";
        
        if (!node.classList.contains("dragging")) {
            node.classList.add("drag-over");
        }
    }

    var handleDragLeave = (e) => node.classList.remove("drag-over");

    var handleDrop = (e) => {
        e.preventDefault();
        node.classList.remove("drag-over");

        var fromIndex = parseInt(e.dataTransfer.getData("text/plain"));
        var parent = node.parentNode;
        var toIndex = Array.from(parent.children).indexOf(node);

        if (fromIndex === -1 || toIndex === -1 || fromIndex === toIndex) return;

        const rect = node.getBoundingClientRect();
        const isBottomHalf = (e.clientY - rect.top) > (rect.height / 2);

        if (fromIndex < toIndex && !isBottomHalf) {
            toIndex--;
        } else if (fromIndex > toIndex && isBottomHalf) {
            toIndex++;
        }
        if (fromIndex !== toIndex) {
            handleReorder?.(fromIndex, toIndex);
        }
    }

    node.setAttribute('draggable', 'true');
    node.addEventListener('dragstart', handleDragStart);
    node.addEventListener('dragend', handleDragEnd);
    node.addEventListener('dragover', handleDragOver);
    node.addEventListener('dragleave', handleDragLeave);
    node.addEventListener('drop', handleDrop);

    return {
        update(newParams) {
            handleReorder = newParams.handleReorder;
        },
        destroy() {
            node.removeEventListener('dragstart', handleDragStart);
            node.removeEventListener('dragend', handleDragEnd);
            node.removeEventListener('dragover', handleDragOver);
            node.removeEventListener('dragleave', handleDragLeave);
            node.removeEventListener('drop', handleDrop);
        }
    }
};

export var hoverable = (node, { item, withCallbackFn }) => {
    var carried = null;
    var isHovered = false;

    var handlePointerEnter = (e) => {
        if (!isHovered && withCallbackFn instanceof Function) {
            node.classList.add("tr-hover");
            carried = withCallbackFn(item);
            isHovered = true;
        }
    };

    var handlePointerLeave = (e) => {
        isHovered = false;
        node.classList.remove("tr-hover")
        if (carried instanceof Function)
            carried();

        carried = null;
    }

    node.addEventListener("pointerenter", handlePointerEnter);
    node.addEventListener("pointerleave", handlePointerLeave);

    return {
        update(newParams) {
            item = newParams.item;
            withCallbackFn = newParams.withCallbackFn;
        },
        destroy() {
            if (isHovered && carried instanceof Function) {
                carried();
            }
            node.removeEventListener('pointerenter', handlePointerEnter);
            node.removeEventListener('pointerleave', handlePointerLeave);
        }
    }
}