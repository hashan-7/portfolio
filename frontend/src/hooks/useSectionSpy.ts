import { useEffect, useRef, useState } from 'react';

export function useSectionSpy(sectionIds: string[]): string {
  const [activeSection, setActiveSection] = useState(sectionIds[0] ?? 'home');
  const visibilityById = useRef(new Map<string, number>());

  useEffect(() => {
    if (sectionIds.length === 0) {
      return undefined;
    }

    const visibilityMap = visibilityById.current;
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          visibilityMap.set(entry.target.id, entry.isIntersecting ? entry.intersectionRatio : 0);
        }

        const nextSection = sectionIds.reduce((bestId, sectionId) => {
          const bestVisibility = visibilityMap.get(bestId) ?? 0;
          const sectionVisibility = visibilityMap.get(sectionId) ?? 0;
          return sectionVisibility > bestVisibility ? sectionId : bestId;
        }, sectionIds[0]);

        if ((visibilityMap.get(nextSection) ?? 0) > 0) {
          setActiveSection((currentSection) =>
            currentSection === nextSection ? currentSection : nextSection,
          );
        }
      },
      {
        rootMargin: '-18% 0px -62% 0px',
        threshold: [0, 0.15, 0.35, 0.6],
      },
    );

    const sections = sectionIds
      .map((sectionId) => document.getElementById(sectionId))
      .filter((section): section is HTMLElement => Boolean(section));

    sections.forEach((section) => observer.observe(section));

    return () => {
      observer.disconnect();
      visibilityMap.clear();
    };
  }, [sectionIds]);

  return activeSection;
}
