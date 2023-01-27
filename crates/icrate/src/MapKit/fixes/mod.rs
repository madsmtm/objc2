use crate::MapKit::*;

extern_static!(MKFeatureDisplayPriorityRequired: MKFeatureDisplayPriority = 1000.);
extern_static!(MKFeatureDisplayPriorityDefaultHigh: MKFeatureDisplayPriority = 750.);
extern_static!(MKFeatureDisplayPriorityDefaultLow: MKFeatureDisplayPriority = 250.);
extern_static!(MKAnnotationViewZPriorityMax: MKAnnotationViewZPriority = 1000.);
extern_static!(MKAnnotationViewZPriorityDefaultSelected: MKAnnotationViewZPriority = 1000.);
extern_static!(MKAnnotationViewZPriorityDefaultUnselected: MKAnnotationViewZPriority = 500.);
extern_static!(MKAnnotationViewZPriorityMin: MKAnnotationViewZPriority = 0.);

#[cfg(feature = "MapKit_MKMapItem")]
unsafe impl crate::Foundation::NSCoding for MKMapItem {}
