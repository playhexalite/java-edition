package org.hexalite.network.kraken.gameplay.feature.item

import net.kyori.adventure.text.Component

sealed class CustomItemModifier {
    data class Name(val component: Component): CustomItemModifier()

    data class Lore(val components: List<Component>): CustomItemModifier()

    data class AttackSpeed(val value: Double): CustomItemModifier()

    data class AttackDamage(val value: Double): CustomItemModifier()

    data class AttackKnockback(val amount: Double): CustomItemModifier()

    data class Armor(val points: Double): CustomItemModifier()

    data class KnockbackResistance(val amount: Double): CustomItemModifier()
}