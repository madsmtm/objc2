// We probably technically had the choice of making these classes mutating
// methods require `&mut`, but as with so many things in Cocoa, that would
// make it difficult to use in a larger context (e.g. even after assigning a
// layer to a view you'd often still want to modify the layer).

data! {
    class CALayer {
        unsafe +layer;
        unsafe -init;

        // -initWithLayer must be called in a specific context

        // -presentationLayer's return value may not be safe to modify
        // -modelLayer must be called in a specific context

        unsafe -bounds;
        unsafe -setBounds;
        unsafe -position;
        unsafe -setPosition;
        unsafe -zPosition;
        unsafe -setZPosition;
        unsafe -anchorPoint;
        unsafe -setAnchorPoint;
        unsafe -anchorPointZ;
        unsafe -setAnchorPointZ;
        unsafe -transform;
        unsafe -setTransform;
        unsafe -affineTransform;
        unsafe -setAffineTransform;
        unsafe -frame;
        unsafe -setFrame;
        unsafe -isHidden;
        unsafe -setHidden;
        unsafe -isDoubleSided;
        unsafe -setDoubleSided;
        unsafe -isGeometryFlipped;
        unsafe -setGeometryFlipped;

        unsafe -contentsAreFlipped;

        unsafe -superlayer;
        unsafe -removeFromSuperlayer;

        // -sublayers is not safe, since the returned array is not guaranteed
        // to retain its elements.

        // -setSublayers not safe since it requires all layers to have a
        // `nil` superlayer.

        // If the layer already has a superlayer, it will be changed
        // appropriately by these methods (effectively, removeFromSuperlayer
        // is called on the given layer inside these).
        unsafe -addSublayer;
        unsafe -insertSublayer_atIndex;
        unsafe -insertSublayer_below;
        unsafe -insertSublayer_above;

        // -replaceSublayer is not safe since it requires `oldlayer` to exist
        // in the current layer.

        unsafe -sublayerTransform;
        unsafe -setSublayerTransform;
        unsafe -mask;
        // -setMask's argument must have a `nil` superlayer
        unsafe -masksToBounds;
        unsafe -setMasksToBounds;

        unsafe -convertPoint_fromLayer;
        unsafe -convertPoint_toLayer;
        unsafe -convertRect_fromLayer;
        unsafe -convertRect_toLayer;
        unsafe -convertTime_fromLayer;
        unsafe -convertTime_toLayer;
        unsafe -hitTest;
        unsafe -containsPoint;

        // No type set:
        // -contents;
        // -setContents;
        unsafe -contentsRect;
        unsafe -setContentsRect;
        unsafe -contentsGravity;
        unsafe -setContentsGravity;
        unsafe -contentsScale;
        unsafe -setContentsScale;
        unsafe -contentsCenter;
        unsafe -setContentsCenter;
        unsafe -contentsFormat;
        unsafe -setContentsFormat;
        unsafe -minificationFilter;
        unsafe -setMinificationFilter;
        unsafe -magnificationFilter;
        unsafe -setMagnificationFilter;
        unsafe -minificationFilterBias;
        unsafe -setMinificationFilterBias;
        unsafe -isOpaque;
        unsafe -setOpaque;

        unsafe -display;
        unsafe -setNeedsDisplay;
        unsafe -setNeedsDisplayInRect;
        unsafe -needsDisplay;
        unsafe -displayIfNeeded;
        unsafe -needsDisplayOnBoundsChange;
        unsafe -setNeedsDisplayOnBoundsChange;

        unsafe -drawsAsynchronously;
        unsafe -setDrawsAsynchronously;
        unsafe -edgeAntialiasingMask;
        unsafe -setEdgeAntialiasingMask;
        unsafe -allowsEdgeAntialiasing;
        unsafe -setAllowsEdgeAntialiasing;
        unsafe -cornerRadius;
        unsafe -setCornerRadius;
        unsafe -maskedCorners;
        unsafe -setMaskedCorners;
        unsafe -cornerCurve;
        unsafe -setCornerCurve;
        unsafe -cornerCurveExpansionFactor;
        unsafe -borderWidth;
        unsafe -setBorderWidth;
        unsafe -opacity;
        // Gives "undefined results" outside [0; 1], but by this the authors
        // very likely didn't mean "triggers language-level UB".
        unsafe -setOpacity;
        unsafe -allowsGroupOpacity;
        unsafe -setAllowsGroupOpacity;
        // No type set:
        // -compositingFilter;
        // -setCompositingFilter;
        // -filters;
        // -setFilters;
        // -backgroundFilters;
        // -setBackgroundFilters;
        unsafe -shouldRasterize;
        unsafe -setShouldRasterize;
        unsafe -rasterizationScale;
        unsafe -setRasterizationScale;
        unsafe -shadowOpacity;
        // Gives "undefined results" outside [0; 1], but by this the authors
        // very likely didn't mean "triggers language-level UB".
        unsafe -setShadowOpacity;
        unsafe -shadowOffset;
        unsafe -setShadowOffset;
        unsafe -shadowRadius;
        unsafe -setShadowRadius;
        unsafe -autoresizingMask;
        unsafe -setAutoresizingMask;
        unsafe -layoutManager;
        unsafe -setLayoutManager;

        unsafe -preferredFrameSize;
        unsafe -setNeedsLayout;
        unsafe -needsLayout;
        unsafe -layoutIfNeeded;
        unsafe -layoutSublayers;
        unsafe -resizeSublayersWithOldSize;
        unsafe -resizeWithOldSuperlayerSize;

        unsafe -defaultActionForKey;
        unsafe -actionForKey;
        unsafe -actions;
        unsafe -setActions;

        // Copies the animation
        unsafe -addAnimation_forKey;

        unsafe -removeAllAnimations;
        unsafe -removeAnimationForKey;
        unsafe -animationKeys;

        // Not safe since modifying the returned animation is UB.
        // -animationForKey;

        unsafe -name;
        unsafe -setName;
        unsafe -delegate;
        unsafe -setDelegate;
        // No type set:
        // -style;
        // -setStyle;
    }

    class CARenderer {
        unsafe -layer;
        unsafe -setLayer;
        unsafe -bounds;
        unsafe -setBounds;
        unsafe -updateBounds;
        unsafe -addUpdateRect;
        unsafe -render;
        unsafe -nextFrameTime;
        unsafe -endFrame;
    }

    // SAFETY: CATransaction is basically just a way to call methods that
    // access thread-local state.
    class CATransaction {
        unsafe +begin;
        unsafe +commit;
        unsafe +flush;

        unsafe +animationDuration;
        unsafe +setAnimationDuration;

        unsafe +animationTimingFunction;
        unsafe +setAnimationTimingFunction;

        unsafe +disableActions;
        unsafe +setDisableActions;

        // TODO:
        // unsafe +completionBlock;
        // unsafe +setCompletionBlock;
    }

    unsafe fn CACurrentMediaTime;

    // SAFETY: Basic mathematical functions
    unsafe fn CATransform3DIsIdentity;
    unsafe fn CATransform3DEqualToTransform;
    unsafe fn CATransform3DMakeTranslation;
    unsafe fn CATransform3DMakeScale;
    unsafe fn CATransform3DMakeRotation;
    unsafe fn CATransform3DTranslate;
    unsafe fn CATransform3DScale;
    unsafe fn CATransform3DRotate;
    unsafe fn CATransform3DConcat;
    unsafe fn CATransform3DInvert;
    unsafe fn CATransform3DMakeAffineTransform;
    unsafe fn CATransform3DIsAffine;
    unsafe fn CATransform3DGetAffineTransform;

    class CAMetalLayer {
        // TODO
    }
}
