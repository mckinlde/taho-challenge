# TAHO Code Exercise

Welcome to the TAHO code exercise! We look forward to seeing how you approach problems and structure solutions.

This exercise should take 1-2 hours to complete. We've intentionally left many details open-ended so you can focus on what matters most and maybe have some fun too.


## Description

The deliverable is a Rust crate that **finds the shortest route between locations in space**.

**Locations** are unique, fixed points in a space where distance can be calculated. Each location can have zero or more **connections** to other locations -- think of supply station waypoints embedded throughout outer space (or hyperspace).

**Connection distance** is derived solely from location coordinates. The connection itself has no additional attributes. In our outer space example, all paths would be straight lines between stations.

A **route** is an ordered sequence of locations where each step moves between connected locations. The **shortest route** minimizes total distance compared to all other possible paths.

Design your API as if it's destined for production use at TAHO, where we need to recalculate optimal routes when a network of locations and connections changes.


## What We're Looking For

- Thoughtful design and use of data structures
- Efficient time management and focused scope
- Clear, well-organized, and readable code
- Idiomatic Rust patterns and conventions
- Intuitive and ergonomic API design
- Clean version control practices


## What Doesn't Matter

**Coordinate system:** Use any system that supports distance calculation between points. Pick something interesting if you want, but don't let it consume your time budget.

**Result presentation:** You're building a library, not a user interface. Focus on the core functionality.

## Tips

- Avoid external crates when possible. This gives us more opportunity to see how you design components within a system.
- AI assistants are fine, but be ready to walk through and explain your decisions. Include your prompts and plans in version control.
- Clarity is more important than performance. When in doubt, go for the cleaner solution.
