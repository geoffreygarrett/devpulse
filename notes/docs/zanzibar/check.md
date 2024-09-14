To represent the full mathematical notation for the ReBAC (Relationship-Based Access Control) authorization check using Zanzibar principles, considering userset rewrites and different relation types like `owner`, `editor`, and `viewer`, we need to extend the simple case to incorporate these additional rules.

Here is the detailed mathematical notation:

### Basic Notation

1. **Check Definition**:
   \[
   \text{CHECK}(U, \langle \text{object#relation} \rangle) =
   \exists \text{tuple} \langle \text{object#relation@U} \rangle \vee
   \exists \text{tuple} \langle \text{object#relation@U'} \rangle \text{ where }
   U' = \langle \text{object'}#\text{relation'} \rangle \text{ such that } \text{CHECK}(U, U')
   \]

### Extended Notation for Userset Rewrites

1. **Doc Relations**:

    - **Owner**:
      \[
      \text{CHECK}(U, \langle \text{doc#owner} \rangle) =
      \exists \text{tuple} \langle \text{doc#owner@U} \rangle
      \]

    - **Editor**:
      \[
      \text{CHECK}(U, \langle \text{doc#editor} \rangle) =
      \exists \text{tuple} \langle \text{doc#editor@U} \rangle \vee
      \text{CHECK}(U, \langle \text{doc#owner} \rangle)
      \]

    - **Viewer**:
      \[
      \text{CHECK}(U, \langle \text{doc#viewer} \rangle) =
      \exists \text{tuple} \langle \text{doc#viewer@U} \rangle \vee
      \text{CHECK}(U, \langle \text{doc#editor} \rangle) \vee
      \exists \text{tuple} \langle \text{doc#parent@T} \rangle \text{ where }
      \text{CHECK}(U, \langle T#viewer \rangle)
      \]

### Combining Rewrites and Basic Check

To handle cases where the user might have indirect access through rewritten usersets, we need to expand the CHECK function recursively:

$$
\begin{aligned}
\text{CHECK}(U, \langle \text{object#relation} \rangle) =
\begin{cases}
\exists \text{tuple} \langle \text{object#owner@U} \rangle & \text{if relation is owner} \\
\exists \text{tuple} \langle \text{object#editor@U} \rangle \vee \text{CHECK}(U, \langle \text{object#owner} \rangle) & \text{if relation is editor} \\
\exists \text{tuple} \langle \text{object#viewer@U} \rangle \vee \text{CHECK}(U, \langle \text{object#editor} \rangle) \vee \exists \text{tuple} \langle \text{object#parent@T} \rangle \text{ where } \text{CHECK}(U, \langle T#viewer \rangle) & \text{if relation is viewer} \\
\exists \text{tuple} \langle \text{object#relation@U} \rangle \vee \exists \text{tuple} \langle \text{object#relation@U'} \rangle \text{ where } U' = \langle \text{object'}#\text{relation'} \rangle \text{ such that } \text{CHECK}(U, U') & \text{otherwise}
\end{cases}
\end{aligned}
$$

### Summary

The full mathematical expression for ReBAC auth checks, incorporating various relations and userset rewrites, is as follows:

1. **Owner Relation**:
   \[
   \text{CHECK}(U, \langle \text{doc#owner} \rangle) =
   \exists \text{tuple} \langle \text{doc#owner@U} \rangle
   \]

2. **Editor Relation**:
   \[
   \text{CHECK}(U, \langle \text{doc#editor} \rangle) =
   \exists \text{tuple} \langle \text{doc#editor@U} \rangle \vee
   \text{CHECK}(U, \langle \text{doc#owner} \rangle)
   \]

3. **Viewer Relation**:
   \[
   \text{CHECK}(U, \langle \text{doc#viewer} \rangle) =
   \exists \text{tuple} \langle \text{doc#viewer@U} \rangle \vee
   \text{CHECK}(U, \langle \text{doc#editor} \rangle) \vee
   \exists \text{tuple} \langle \text{doc#parent@T} \rangle \text{ where }
   \text{CHECK}(U, \langle T#viewer \rangle)
   \]